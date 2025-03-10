# RustMMO

## 사용법
### 서버
mmp_prototype안에 있는 프로젝트가 서버 입니다.

```bash
cd mmp_prototype
cargo run
```

위와 같은 방법으로 빌드와 실행이 가능합니다.

### Virtual Client
virtual_agent 안에 있는 프로젝트가 가상 클라이언트입니다.
현재 총 100개의 가상 클라이언트를 생성합니다. 서버를 우선 실행시켜놔야 잘 동작합니다.

```bash
cd virtual_agent
cargo run
```

위와 같은 방법으로 빌드와 실행이 가능합니다.

## 사용된 crates
### flatbuffer
구글에서 개발한 데이터 직렬화 툴인 flatbuffer를 사용했습니다.

https://flatbuffers.dev/

rust를 포함해 많은 언어를 지원해주고 구글 `protobuf`에 비해 빠른 속도를 보여줍니다.

### tokio
비동기 프로그래밍에 유용한 crate입니다.
go언어의 `goroutine`처럼 경량스레드를 생성해줍니다.

또한 간단한 tcp/udp 통신 함수들도 있어 stateful 서버를 만드는데 적합한 라이브러리입니다.

### rand
여러 랜덤함수를 포함하는 crate입니다.
rust는 std에 rand함수가 없어 대신 이 crate를 사용합니다.


## dsnet
직접 구현한 stateful 게임서버 라이브러리입니다.

### 구조
각 accept한 session별로 Send용과 Recv용 Task두개를 생성합니다.
그리고 모든 콜백처리(Accept, Recv, Disconnect 등)을 처리할 채널을 만들었습니다.
`get_callback`함수를 통해 발생한 콜백들을 처리할 수 있습니다.

main쓰레드를 `dsnet`내부에서 돌리는 것보다는 외부 메인쓰레드에서 처리하도록 하는게 좋을 것 같아 이런 구조로 작업했습니다.

#### send
각 Session별로 Send용 mpsc 채널을 가집니다.
send용 채널을 통해 `Arc<[u8]>`로 된 데이터를 보내면 `Send_process`에서 전송합니다.
Nagle알고리즘을 적용하여 최적화된 전송을 보내주며 `write_vectored`함수를 이용해 Zerocopy로 전송되도록 했습니다.

#### receive
Recv부분에서는 우선 읽은 데이터를 `VecDeque<u8>` 링버퍼에 저장합니다.
그후 패킷해더를 검사하여 모두 도착했을 경우 그 부분을 `Vec<u8>`로 만들어 get_callback함수로 보냅니다.


## 메인 루프
메인 루프는 크게 4 단계로 이루어집니다.
- `dsnet` 으로부터 콜백 수신 및 처리
- `GameUser` -> `World` 로 되어있는 mpsc 채널을 통해 `WorldCommand` 전달
- `World` 갱신
- `World` -> `GameUser`으로 연결되어있는 여러 spsc채널을 통해 `WorldResponse` 전달
  - 각 월드 변경점을 브로드캐스팅 등으로 각 세션에전달. 세션에서 각 클라이언트로 데이터 전송

### GameUser
`GameUser`는 dsnet의 Session을 소유하고 있는 객체입니다.

클라이언트에서 받은 패킷을 처리하고 월드의 변경점을 클라에 전송하는 역할을 맡습니다.

### World
`World`는 게임의 로직을 맡을 객체입니다. 여러 `Actor`의 처리를 담당합니다.

`WorldCommand`를 처리하고 월드의 변경점을 `WorldResponse`를 통해 GameUser에 전달합니다.

## 테스트 클라이언트
[링크](https://1drv.ms/u/c/c07419687adffcbf/EVhBO0vJoM1FpQjh7oyeuWMBIqZ5G4lIC3ufhDnuoWINkg?e=iaQpYE)

위 링크를 통해 클라이언트를 설치후 실행하면 실제 어떻게 동작하는지를 볼 수 있습니다.

zip파일을 압축 풀고 Nexus.exe파일을 실행하시면 됩니다.

아직 아이피를 127.0.0.1로 고정해두어 한 머신에서만 테스트 가능합니다.

![test_client](/readme_resource/test_client.webp)

- mmp_server와 virtual_agent를 실행한 뒤 테스트 클라이언트를 실행한 모습입니다.
- 화면에 나타난 정육면체들은 접속한 클라이언트의 플레이어블 캐릭터입니다.
- virtual_agent를 통해 생성된 정육면체들은 랜덤한 색상을 가지고 랜덤하게 움직이고 있습니다.
- 별도 Test client에 의해 접속한 정육면체는 빨간색이며, 상하좌우 방향키로 움직일 수 있습니다.
