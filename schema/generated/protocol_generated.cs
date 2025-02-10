// <auto-generated>
//  automatically generated by the FlatBuffers compiler, do not modify
// </auto-generated>

namespace Nexus
{

using global::System;
using global::System.Collections.Generic;
using global::Google.FlatBuffers;

public enum ServerCode : ulong
{
  SUCCESS = 0,
  FAILED = 1,
};

public enum PacketType : ushort
{
  CG_LOGIN_REQ = 0,
  GC_LOGIN_RES = 1,
  GC_SPAWN_ACTOR_NOTI = 2,
  CG_CHANGE_MOVE_DIRECTION_REQ = 3,
  GC_CHANGE_MOVE_DIRECTION_RES = 4,
  GC_CHANGE_ACTOR_DIRECTION_NOTI = 5,
};

public struct Color : IFlatbufferObject
{
  private Struct __p;
  public ByteBuffer ByteBuffer { get { return __p.bb; } }
  public void __init(int _i, ByteBuffer _bb) { __p = new Struct(_i, _bb); }
  public Color __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public sbyte R { get { return __p.bb.GetSbyte(__p.bb_pos + 0); } }
  public sbyte G { get { return __p.bb.GetSbyte(__p.bb_pos + 1); } }
  public sbyte B { get { return __p.bb.GetSbyte(__p.bb_pos + 2); } }
  public sbyte A { get { return __p.bb.GetSbyte(__p.bb_pos + 3); } }

  public static Offset<Nexus.Color> CreateColor(FlatBufferBuilder builder, sbyte R, sbyte G, sbyte B, sbyte A) {
    builder.Prep(1, 4);
    builder.PutSbyte(A);
    builder.PutSbyte(B);
    builder.PutSbyte(G);
    builder.PutSbyte(R);
    return new Offset<Nexus.Color>(builder.Offset);
  }
}

public struct Vec2 : IFlatbufferObject
{
  private Struct __p;
  public ByteBuffer ByteBuffer { get { return __p.bb; } }
  public void __init(int _i, ByteBuffer _bb) { __p = new Struct(_i, _bb); }
  public Vec2 __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public float X { get { return __p.bb.GetFloat(__p.bb_pos + 0); } }
  public float Y { get { return __p.bb.GetFloat(__p.bb_pos + 4); } }

  public static Offset<Nexus.Vec2> CreateVec2(FlatBufferBuilder builder, float X, float Y) {
    builder.Prep(4, 8);
    builder.PutFloat(Y);
    builder.PutFloat(X);
    return new Offset<Nexus.Vec2>(builder.Offset);
  }
}

public struct CGLoginReq : IFlatbufferObject
{
  private Table __p;
  public ByteBuffer ByteBuffer { get { return __p.bb; } }
  public static void ValidateVersion() { FlatBufferConstants.FLATBUFFERS_25_1_24(); }
  public static CGLoginReq GetRootAsCGLoginReq(ByteBuffer _bb) { return GetRootAsCGLoginReq(_bb, new CGLoginReq()); }
  public static CGLoginReq GetRootAsCGLoginReq(ByteBuffer _bb, CGLoginReq obj) { return (obj.__assign(_bb.GetInt(_bb.Position) + _bb.Position, _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __p = new Table(_i, _bb); }
  public CGLoginReq __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public string Name { get { int o = __p.__offset(4); return o != 0 ? __p.__string(o + __p.bb_pos) : null; } }
#if ENABLE_SPAN_T
  public Span<byte> GetNameBytes() { return __p.__vector_as_span<byte>(4, 1); }
#else
  public ArraySegment<byte>? GetNameBytes() { return __p.__vector_as_arraysegment(4); }
#endif
  public byte[] GetNameArray() { return __p.__vector_as_array<byte>(4); }
  public Nexus.Color? Color { get { int o = __p.__offset(6); return o != 0 ? (Nexus.Color?)(new Nexus.Color()).__assign(o + __p.bb_pos, __p.bb) : null; } }

  public static void StartCGLoginReq(FlatBufferBuilder builder) { builder.StartTable(2); }
  public static void AddName(FlatBufferBuilder builder, StringOffset nameOffset) { builder.AddOffset(0, nameOffset.Value, 0); }
  public static void AddColor(FlatBufferBuilder builder, Offset<Nexus.Color> colorOffset) { builder.AddStruct(1, colorOffset.Value, 0); }
  public static Offset<Nexus.CGLoginReq> EndCGLoginReq(FlatBufferBuilder builder) {
    int o = builder.EndTable();
    return new Offset<Nexus.CGLoginReq>(o);
  }
}


static public class CGLoginReqVerify
{
  static public bool Verify(Google.FlatBuffers.Verifier verifier, uint tablePos)
  {
    return verifier.VerifyTableStart(tablePos)
      && verifier.VerifyString(tablePos, 4 /*Name*/, false)
      && verifier.VerifyField(tablePos, 6 /*Color*/, 4 /*Nexus.Color*/, 1, false)
      && verifier.VerifyTableEnd(tablePos);
  }
}
public struct GCLoginRes : IFlatbufferObject
{
  private Table __p;
  public ByteBuffer ByteBuffer { get { return __p.bb; } }
  public static void ValidateVersion() { FlatBufferConstants.FLATBUFFERS_25_1_24(); }
  public static GCLoginRes GetRootAsGCLoginRes(ByteBuffer _bb) { return GetRootAsGCLoginRes(_bb, new GCLoginRes()); }
  public static GCLoginRes GetRootAsGCLoginRes(ByteBuffer _bb, GCLoginRes obj) { return (obj.__assign(_bb.GetInt(_bb.Position) + _bb.Position, _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __p = new Table(_i, _bb); }
  public GCLoginRes __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public ulong ActorIdx { get { int o = __p.__offset(4); return o != 0 ? __p.bb.GetUlong(o + __p.bb_pos) : (ulong)0; } }
  public Nexus.ServerCode Result { get { int o = __p.__offset(6); return o != 0 ? (Nexus.ServerCode)__p.bb.GetUlong(o + __p.bb_pos) : Nexus.ServerCode.SUCCESS; } }

  public static Offset<Nexus.GCLoginRes> CreateGCLoginRes(FlatBufferBuilder builder,
      ulong actor_idx = 0,
      Nexus.ServerCode result = Nexus.ServerCode.SUCCESS) {
    builder.StartTable(2);
    GCLoginRes.AddResult(builder, result);
    GCLoginRes.AddActorIdx(builder, actor_idx);
    return GCLoginRes.EndGCLoginRes(builder);
  }

  public static void StartGCLoginRes(FlatBufferBuilder builder) { builder.StartTable(2); }
  public static void AddActorIdx(FlatBufferBuilder builder, ulong actorIdx) { builder.AddUlong(0, actorIdx, 0); }
  public static void AddResult(FlatBufferBuilder builder, Nexus.ServerCode result) { builder.AddUlong(1, (ulong)result, 0); }
  public static Offset<Nexus.GCLoginRes> EndGCLoginRes(FlatBufferBuilder builder) {
    int o = builder.EndTable();
    return new Offset<Nexus.GCLoginRes>(o);
  }
}


static public class GCLoginResVerify
{
  static public bool Verify(Google.FlatBuffers.Verifier verifier, uint tablePos)
  {
    return verifier.VerifyTableStart(tablePos)
      && verifier.VerifyField(tablePos, 4 /*ActorIdx*/, 8 /*ulong*/, 8, false)
      && verifier.VerifyField(tablePos, 6 /*Result*/, 8 /*Nexus.ServerCode*/, 8, false)
      && verifier.VerifyTableEnd(tablePos);
  }
}
public struct GCSpawnActorNoti : IFlatbufferObject
{
  private Table __p;
  public ByteBuffer ByteBuffer { get { return __p.bb; } }
  public static void ValidateVersion() { FlatBufferConstants.FLATBUFFERS_25_1_24(); }
  public static GCSpawnActorNoti GetRootAsGCSpawnActorNoti(ByteBuffer _bb) { return GetRootAsGCSpawnActorNoti(_bb, new GCSpawnActorNoti()); }
  public static GCSpawnActorNoti GetRootAsGCSpawnActorNoti(ByteBuffer _bb, GCSpawnActorNoti obj) { return (obj.__assign(_bb.GetInt(_bb.Position) + _bb.Position, _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __p = new Table(_i, _bb); }
  public GCSpawnActorNoti __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public ulong ActorIdx { get { int o = __p.__offset(4); return o != 0 ? __p.bb.GetUlong(o + __p.bb_pos) : (ulong)0; } }
  public Nexus.Color? Color { get { int o = __p.__offset(6); return o != 0 ? (Nexus.Color?)(new Nexus.Color()).__assign(o + __p.bb_pos, __p.bb) : null; } }
  public float Speed { get { int o = __p.__offset(8); return o != 0 ? __p.bb.GetFloat(o + __p.bb_pos) : (float)0.0f; } }
  public Nexus.Vec2? Position { get { int o = __p.__offset(10); return o != 0 ? (Nexus.Vec2?)(new Nexus.Vec2()).__assign(o + __p.bb_pos, __p.bb) : null; } }
  public Nexus.Vec2? Direction { get { int o = __p.__offset(12); return o != 0 ? (Nexus.Vec2?)(new Nexus.Vec2()).__assign(o + __p.bb_pos, __p.bb) : null; } }

  public static void StartGCSpawnActorNoti(FlatBufferBuilder builder) { builder.StartTable(5); }
  public static void AddActorIdx(FlatBufferBuilder builder, ulong actorIdx) { builder.AddUlong(0, actorIdx, 0); }
  public static void AddColor(FlatBufferBuilder builder, Offset<Nexus.Color> colorOffset) { builder.AddStruct(1, colorOffset.Value, 0); }
  public static void AddSpeed(FlatBufferBuilder builder, float speed) { builder.AddFloat(2, speed, 0.0f); }
  public static void AddPosition(FlatBufferBuilder builder, Offset<Nexus.Vec2> positionOffset) { builder.AddStruct(3, positionOffset.Value, 0); }
  public static void AddDirection(FlatBufferBuilder builder, Offset<Nexus.Vec2> directionOffset) { builder.AddStruct(4, directionOffset.Value, 0); }
  public static Offset<Nexus.GCSpawnActorNoti> EndGCSpawnActorNoti(FlatBufferBuilder builder) {
    int o = builder.EndTable();
    return new Offset<Nexus.GCSpawnActorNoti>(o);
  }
}


static public class GCSpawnActorNotiVerify
{
  static public bool Verify(Google.FlatBuffers.Verifier verifier, uint tablePos)
  {
    return verifier.VerifyTableStart(tablePos)
      && verifier.VerifyField(tablePos, 4 /*ActorIdx*/, 8 /*ulong*/, 8, false)
      && verifier.VerifyField(tablePos, 6 /*Color*/, 4 /*Nexus.Color*/, 1, false)
      && verifier.VerifyField(tablePos, 8 /*Speed*/, 4 /*float*/, 4, false)
      && verifier.VerifyField(tablePos, 10 /*Position*/, 8 /*Nexus.Vec2*/, 4, false)
      && verifier.VerifyField(tablePos, 12 /*Direction*/, 8 /*Nexus.Vec2*/, 4, false)
      && verifier.VerifyTableEnd(tablePos);
  }
}
public struct CGChangeMoveDirectionReq : IFlatbufferObject
{
  private Table __p;
  public ByteBuffer ByteBuffer { get { return __p.bb; } }
  public static void ValidateVersion() { FlatBufferConstants.FLATBUFFERS_25_1_24(); }
  public static CGChangeMoveDirectionReq GetRootAsCGChangeMoveDirectionReq(ByteBuffer _bb) { return GetRootAsCGChangeMoveDirectionReq(_bb, new CGChangeMoveDirectionReq()); }
  public static CGChangeMoveDirectionReq GetRootAsCGChangeMoveDirectionReq(ByteBuffer _bb, CGChangeMoveDirectionReq obj) { return (obj.__assign(_bb.GetInt(_bb.Position) + _bb.Position, _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __p = new Table(_i, _bb); }
  public CGChangeMoveDirectionReq __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public Nexus.Vec2? Direction { get { int o = __p.__offset(4); return o != 0 ? (Nexus.Vec2?)(new Nexus.Vec2()).__assign(o + __p.bb_pos, __p.bb) : null; } }

  public static void StartCGChangeMoveDirectionReq(FlatBufferBuilder builder) { builder.StartTable(1); }
  public static void AddDirection(FlatBufferBuilder builder, Offset<Nexus.Vec2> directionOffset) { builder.AddStruct(0, directionOffset.Value, 0); }
  public static Offset<Nexus.CGChangeMoveDirectionReq> EndCGChangeMoveDirectionReq(FlatBufferBuilder builder) {
    int o = builder.EndTable();
    return new Offset<Nexus.CGChangeMoveDirectionReq>(o);
  }
}


static public class CGChangeMoveDirectionReqVerify
{
  static public bool Verify(Google.FlatBuffers.Verifier verifier, uint tablePos)
  {
    return verifier.VerifyTableStart(tablePos)
      && verifier.VerifyField(tablePos, 4 /*Direction*/, 8 /*Nexus.Vec2*/, 4, false)
      && verifier.VerifyTableEnd(tablePos);
  }
}
public struct GCChangeMoveDirectionRes : IFlatbufferObject
{
  private Table __p;
  public ByteBuffer ByteBuffer { get { return __p.bb; } }
  public static void ValidateVersion() { FlatBufferConstants.FLATBUFFERS_25_1_24(); }
  public static GCChangeMoveDirectionRes GetRootAsGCChangeMoveDirectionRes(ByteBuffer _bb) { return GetRootAsGCChangeMoveDirectionRes(_bb, new GCChangeMoveDirectionRes()); }
  public static GCChangeMoveDirectionRes GetRootAsGCChangeMoveDirectionRes(ByteBuffer _bb, GCChangeMoveDirectionRes obj) { return (obj.__assign(_bb.GetInt(_bb.Position) + _bb.Position, _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __p = new Table(_i, _bb); }
  public GCChangeMoveDirectionRes __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public Nexus.ServerCode Result { get { int o = __p.__offset(4); return o != 0 ? (Nexus.ServerCode)__p.bb.GetUlong(o + __p.bb_pos) : Nexus.ServerCode.SUCCESS; } }

  public static Offset<Nexus.GCChangeMoveDirectionRes> CreateGCChangeMoveDirectionRes(FlatBufferBuilder builder,
      Nexus.ServerCode result = Nexus.ServerCode.SUCCESS) {
    builder.StartTable(1);
    GCChangeMoveDirectionRes.AddResult(builder, result);
    return GCChangeMoveDirectionRes.EndGCChangeMoveDirectionRes(builder);
  }

  public static void StartGCChangeMoveDirectionRes(FlatBufferBuilder builder) { builder.StartTable(1); }
  public static void AddResult(FlatBufferBuilder builder, Nexus.ServerCode result) { builder.AddUlong(0, (ulong)result, 0); }
  public static Offset<Nexus.GCChangeMoveDirectionRes> EndGCChangeMoveDirectionRes(FlatBufferBuilder builder) {
    int o = builder.EndTable();
    return new Offset<Nexus.GCChangeMoveDirectionRes>(o);
  }
}


static public class GCChangeMoveDirectionResVerify
{
  static public bool Verify(Google.FlatBuffers.Verifier verifier, uint tablePos)
  {
    return verifier.VerifyTableStart(tablePos)
      && verifier.VerifyField(tablePos, 4 /*Result*/, 8 /*Nexus.ServerCode*/, 8, false)
      && verifier.VerifyTableEnd(tablePos);
  }
}
public struct GCChangeActorDirectionNoti : IFlatbufferObject
{
  private Table __p;
  public ByteBuffer ByteBuffer { get { return __p.bb; } }
  public static void ValidateVersion() { FlatBufferConstants.FLATBUFFERS_25_1_24(); }
  public static GCChangeActorDirectionNoti GetRootAsGCChangeActorDirectionNoti(ByteBuffer _bb) { return GetRootAsGCChangeActorDirectionNoti(_bb, new GCChangeActorDirectionNoti()); }
  public static GCChangeActorDirectionNoti GetRootAsGCChangeActorDirectionNoti(ByteBuffer _bb, GCChangeActorDirectionNoti obj) { return (obj.__assign(_bb.GetInt(_bb.Position) + _bb.Position, _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __p = new Table(_i, _bb); }
  public GCChangeActorDirectionNoti __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public ulong ActorIdx { get { int o = __p.__offset(4); return o != 0 ? __p.bb.GetUlong(o + __p.bb_pos) : (ulong)0; } }
  public Nexus.Vec2? Direction { get { int o = __p.__offset(6); return o != 0 ? (Nexus.Vec2?)(new Nexus.Vec2()).__assign(o + __p.bb_pos, __p.bb) : null; } }
  public Nexus.Vec2? Position { get { int o = __p.__offset(8); return o != 0 ? (Nexus.Vec2?)(new Nexus.Vec2()).__assign(o + __p.bb_pos, __p.bb) : null; } }

  public static void StartGCChangeActorDirectionNoti(FlatBufferBuilder builder) { builder.StartTable(3); }
  public static void AddActorIdx(FlatBufferBuilder builder, ulong actorIdx) { builder.AddUlong(0, actorIdx, 0); }
  public static void AddDirection(FlatBufferBuilder builder, Offset<Nexus.Vec2> directionOffset) { builder.AddStruct(1, directionOffset.Value, 0); }
  public static void AddPosition(FlatBufferBuilder builder, Offset<Nexus.Vec2> positionOffset) { builder.AddStruct(2, positionOffset.Value, 0); }
  public static Offset<Nexus.GCChangeActorDirectionNoti> EndGCChangeActorDirectionNoti(FlatBufferBuilder builder) {
    int o = builder.EndTable();
    return new Offset<Nexus.GCChangeActorDirectionNoti>(o);
  }
}


static public class GCChangeActorDirectionNotiVerify
{
  static public bool Verify(Google.FlatBuffers.Verifier verifier, uint tablePos)
  {
    return verifier.VerifyTableStart(tablePos)
      && verifier.VerifyField(tablePos, 4 /*ActorIdx*/, 8 /*ulong*/, 8, false)
      && verifier.VerifyField(tablePos, 6 /*Direction*/, 8 /*Nexus.Vec2*/, 4, false)
      && verifier.VerifyField(tablePos, 8 /*Position*/, 8 /*Nexus.Vec2*/, 4, false)
      && verifier.VerifyTableEnd(tablePos);
  }
}

}
