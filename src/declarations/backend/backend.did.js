export const idlFactory = ({ IDL }) => {
	return IDL.Service({
		hello: IDL.Func([IDL.Text], [IDL.Text], ['query']),
		world: IDL.Func([IDL.Text], [IDL.Text], [])
	});
};
export const init = ({ IDL }) => {
	return [];
};
