@0xec9c07fb4519497a;

interface Theinterf {
	create @0 (data: MaStruct) -> (result: Bool);
	update @1 (name: Text, data: MaStruct) -> (result: MaStruct);
	delete @2 (name: Text) -> (result: Bool);
}

struct MaStruct {
		value @0 :UInt32;
		name @1 :Text;
}
