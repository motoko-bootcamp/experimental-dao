export const idlFactory = ({ IDL }) => {
	const Member = IDL.Record({
		xp: IDL.Nat64,
		principal: IDL.Principal,
		username: IDL.Text,
		level: IDL.Nat64
	});
	const MemberError = IDL.Variant({
		PrincipalAlreadyRegistered: IDL.Null,
		UsernameExists: IDL.Null
	});
	const Result = IDL.Variant({ Ok: IDL.Opt(Member), Err: MemberError });
	return IDL.Service({
		create_member: IDL.Func([IDL.Text, IDL.Nat64, IDL.Nat64], [Result], []),
		get_all_members: IDL.Func([], [IDL.Vec(Member)], ['query'])
	});
};
export const init = ({ IDL }) => {
	return [];
};
