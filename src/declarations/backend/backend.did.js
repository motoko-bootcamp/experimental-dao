export const idlFactory = ({ IDL }) => {
  const Member = IDL.Record({
    'xp' : IDL.Nat64,
    'principal' : IDL.Principal,
    'username' : IDL.Text,
    'level' : IDL.Nat64,
    'roles' : IDL.Nat64,
  });
  const MemberError = IDL.Variant({
    'PrincipalAlreadyRegistered' : IDL.Null,
    'UsernameExists' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Opt(Member), 'Err' : MemberError });
  const HttpRequest = IDL.Record({
    'url' : IDL.Text,
    'method' : IDL.Text,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
  });
  const StreamingStrategy = IDL.Variant({
    'Callback' : IDL.Record({
      'token' : IDL.Record({}),
      'callback' : IDL.Func([], [], ['query']),
    }),
  });
  const HttpResponse = IDL.Record({
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'streaming_strategy' : IDL.Opt(StreamingStrategy),
    'status_code' : IDL.Nat16,
  });
  return IDL.Service({
    'add_to_log' : IDL.Func([IDL.Text], [], []),
    'create_member' : IDL.Func([IDL.Text], [Result], []),
    'get_all_members' : IDL.Func([], [IDL.Vec(Member)], ['query']),
    'get_logs' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], []),
  });
};
export const init = ({ IDL }) => { return []; };
