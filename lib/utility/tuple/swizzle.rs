#[rustfmt::skip]
pub mod s {
pub trait TupleSwizzle2<A, B> {
	fn x(self) -> A;
	fn y(self) -> B;
	fn xx(self) -> (A, A);
	fn xy(self) -> (A, B);
	fn yx(self) -> (B, A);
	fn yy(self) -> (B, B);

	fn r(self) -> A;
	fn g(self) -> B;
	fn rr(self) -> (A, A);
	fn rg(self) -> (A, B);
	fn gr(self) -> (B, A);
	fn gg(self) -> (B, B);
}
pub trait TupleSwizzle3<A, B, C> {
	fn x(self) -> A;
	fn y(self) -> B;
	fn z(self) -> C;
	fn xx(self) -> (A, A);
	fn xy(self) -> (A, B);
	fn xz(self) -> (A, C);
	fn yx(self) -> (B, A);
	fn yy(self) -> (B, B);
	fn yz(self) -> (B, C);
	fn zx(self) -> (C, A);
	fn zy(self) -> (C, B);
	fn zz(self) -> (C, C);
	fn xxx(self) -> (A, A, A);
	fn xxy(self) -> (A, A, B);
	fn xxz(self) -> (A, A, C);
	fn xyx(self) -> (A, B, A);
	fn xyy(self) -> (A, B, B);
	fn xyz(self) -> (A, B, C);
	fn xzx(self) -> (A, C, A);
	fn xzy(self) -> (A, C, B);
	fn xzz(self) -> (A, C, C);
	fn yxx(self) -> (B, A, A);
	fn yxy(self) -> (B, A, B);
	fn yxz(self) -> (B, A, C);
	fn yyx(self) -> (B, B, A);
	fn yyy(self) -> (B, B, B);
	fn yyz(self) -> (B, B, C);
	fn yzx(self) -> (B, C, A);
	fn yzy(self) -> (B, C, B);
	fn yzz(self) -> (B, C, C);
	fn zxx(self) -> (C, A, A);
	fn zxy(self) -> (C, A, B);
	fn zxz(self) -> (C, A, C);
	fn zyx(self) -> (C, B, A);
	fn zyy(self) -> (C, B, B);
	fn zyz(self) -> (C, B, C);
	fn zzx(self) -> (C, C, A);
	fn zzy(self) -> (C, C, B);
	fn zzz(self) -> (C, C, C);

	fn r(self) -> A;
	fn g(self) -> B;
	fn b(self) -> C;
	fn rr(self) -> (A, A);
	fn rg(self) -> (A, B);
	fn rb(self) -> (A, C);
	fn gr(self) -> (B, A);
	fn gg(self) -> (B, B);
	fn gb(self) -> (B, C);
	fn br(self) -> (C, A);
	fn bg(self) -> (C, B);
	fn bb(self) -> (C, C);
	fn rrr(self) -> (A, A, A);
	fn rrg(self) -> (A, A, B);
	fn rrb(self) -> (A, A, C);
	fn rgr(self) -> (A, B, A);
	fn rgg(self) -> (A, B, B);
	fn rgb(self) -> (A, B, C);
	fn rbr(self) -> (A, C, A);
	fn rbg(self) -> (A, C, B);
	fn rbb(self) -> (A, C, C);
	fn grr(self) -> (B, A, A);
	fn grg(self) -> (B, A, B);
	fn grb(self) -> (B, A, C);
	fn ggr(self) -> (B, B, A);
	fn ggg(self) -> (B, B, B);
	fn ggb(self) -> (B, B, C);
	fn gbr(self) -> (B, C, A);
	fn gbg(self) -> (B, C, B);
	fn gbb(self) -> (B, C, C);
	fn brr(self) -> (C, A, A);
	fn brg(self) -> (C, A, B);
	fn brb(self) -> (C, A, C);
	fn bgr(self) -> (C, B, A);
	fn bgg(self) -> (C, B, B);
	fn bgb(self) -> (C, B, C);
	fn bbr(self) -> (C, C, A);
	fn bbg(self) -> (C, C, B);
	fn bbb(self) -> (C, C, C);
}
pub trait TupleSwizzle4<A, B, C, D> {
	fn x(self) -> A;
	fn y(self) -> B;
	fn z(self) -> C;
	fn w(self) -> D;
	fn xx(self) -> (A, A);
	fn xy(self) -> (A, B);
	fn xz(self) -> (A, C);
	fn xw(self) -> (A, D);
	fn yx(self) -> (B, A);
	fn yy(self) -> (B, B);
	fn yz(self) -> (B, C);
	fn yw(self) -> (B, D);
	fn zx(self) -> (C, A);
	fn zy(self) -> (C, B);
	fn zz(self) -> (C, C);
	fn zw(self) -> (C, D);
	fn wx(self) -> (D, A);
	fn wy(self) -> (D, B);
	fn wz(self) -> (D, C);
	fn ww(self) -> (D, D);
	fn xxx(self) -> (A, A, A);
	fn xxy(self) -> (A, A, B);
	fn xxz(self) -> (A, A, C);
	fn xxw(self) -> (A, A, D);
	fn xyx(self) -> (A, B, A);
	fn xyy(self) -> (A, B, B);
	fn xyz(self) -> (A, B, C);
	fn xyw(self) -> (A, B, D);
	fn xzx(self) -> (A, C, A);
	fn xzy(self) -> (A, C, B);
	fn xzz(self) -> (A, C, C);
	fn xzw(self) -> (A, C, D);
	fn xwx(self) -> (A, D, A);
	fn xwy(self) -> (A, D, B);
	fn xwz(self) -> (A, D, C);
	fn xww(self) -> (A, D, D);
	fn yxx(self) -> (B, A, A);
	fn yxy(self) -> (B, A, B);
	fn yxz(self) -> (B, A, C);
	fn yxw(self) -> (B, A, D);
	fn yyx(self) -> (B, B, A);
	fn yyy(self) -> (B, B, B);
	fn yyz(self) -> (B, B, C);
	fn yyw(self) -> (B, B, D);
	fn yzx(self) -> (B, C, A);
	fn yzy(self) -> (B, C, B);
	fn yzz(self) -> (B, C, C);
	fn yzw(self) -> (B, C, D);
	fn ywx(self) -> (B, D, A);
	fn ywy(self) -> (B, D, B);
	fn ywz(self) -> (B, D, C);
	fn yww(self) -> (B, D, D);
	fn zxx(self) -> (C, A, A);
	fn zxy(self) -> (C, A, B);
	fn zxz(self) -> (C, A, C);
	fn zxw(self) -> (C, A, D);
	fn zyx(self) -> (C, B, A);
	fn zyy(self) -> (C, B, B);
	fn zyz(self) -> (C, B, C);
	fn zyw(self) -> (C, B, D);
	fn zzx(self) -> (C, C, A);
	fn zzy(self) -> (C, C, B);
	fn zzz(self) -> (C, C, C);
	fn zzw(self) -> (C, C, D);
	fn zwx(self) -> (C, D, A);
	fn zwy(self) -> (C, D, B);
	fn zwz(self) -> (C, D, C);
	fn zww(self) -> (C, D, D);
	fn wxx(self) -> (D, A, A);
	fn wxy(self) -> (D, A, B);
	fn wxz(self) -> (D, A, C);
	fn wxw(self) -> (D, A, D);
	fn wyx(self) -> (D, B, A);
	fn wyy(self) -> (D, B, B);
	fn wyz(self) -> (D, B, C);
	fn wyw(self) -> (D, B, D);
	fn wzx(self) -> (D, C, A);
	fn wzy(self) -> (D, C, B);
	fn wzz(self) -> (D, C, C);
	fn wzw(self) -> (D, C, D);
	fn wwx(self) -> (D, D, A);
	fn wwy(self) -> (D, D, B);
	fn wwz(self) -> (D, D, C);
	fn www(self) -> (D, D, D);

	fn xxxx(self) -> (A, A, A, A);
	fn xxxy(self) -> (A, A, A, B);
	fn xxxz(self) -> (A, A, A, C);
	fn xxxw(self) -> (A, A, A, D);
	fn xxyx(self) -> (A, A, B, A);
	fn xxyy(self) -> (A, A, B, B);
	fn xxyz(self) -> (A, A, B, C);
	fn xxyw(self) -> (A, A, B, D);
	fn xxzx(self) -> (A, A, C, A);
	fn xxzy(self) -> (A, A, C, B);
	fn xxzz(self) -> (A, A, C, C);
	fn xxzw(self) -> (A, A, C, D);
	fn xxwx(self) -> (A, A, D, A);
	fn xxwy(self) -> (A, A, D, B);
	fn xxwz(self) -> (A, A, D, C);
	fn xxww(self) -> (A, A, D, D);
	fn xyxx(self) -> (A, B, A, A);
	fn xyxy(self) -> (A, B, A, B);
	fn xyxz(self) -> (A, B, A, C);
	fn xyxw(self) -> (A, B, A, D);
	fn xyyx(self) -> (A, B, B, A);
	fn xyyy(self) -> (A, B, B, B);
	fn xyyz(self) -> (A, B, B, C);
	fn xyyw(self) -> (A, B, B, D);
	fn xyzx(self) -> (A, B, C, A);
	fn xyzy(self) -> (A, B, C, B);
	fn xyzz(self) -> (A, B, C, C);
	fn xyzw(self) -> (A, B, C, D);
	fn xywx(self) -> (A, B, D, A);
	fn xywy(self) -> (A, B, D, B);
	fn xywz(self) -> (A, B, D, C);
	fn xyww(self) -> (A, B, D, D);
	fn xzxx(self) -> (A, C, A, A);
	fn xzxy(self) -> (A, C, A, B);
	fn xzxz(self) -> (A, C, A, C);
	fn xzxw(self) -> (A, C, A, D);
	fn xzyx(self) -> (A, C, B, A);
	fn xzyy(self) -> (A, C, B, B);
	fn xzyz(self) -> (A, C, B, C);
	fn xzyw(self) -> (A, C, B, D);
	fn xzzx(self) -> (A, C, C, A);
	fn xzzy(self) -> (A, C, C, B);
	fn xzzz(self) -> (A, C, C, C);
	fn xzzw(self) -> (A, C, C, D);
	fn xzwx(self) -> (A, C, D, A);
	fn xzwy(self) -> (A, C, D, B);
	fn xzwz(self) -> (A, C, D, C);
	fn xzww(self) -> (A, C, D, D);
	fn xwxx(self) -> (A, D, A, A);
	fn xwxy(self) -> (A, D, A, B);
	fn xwxz(self) -> (A, D, A, C);
	fn xwxw(self) -> (A, D, A, D);
	fn xwyx(self) -> (A, D, B, A);
	fn xwyy(self) -> (A, D, B, B);
	fn xwyz(self) -> (A, D, B, C);
	fn xwyw(self) -> (A, D, B, D);
	fn xwzx(self) -> (A, D, C, A);
	fn xwzy(self) -> (A, D, C, B);
	fn xwzz(self) -> (A, D, C, C);
	fn xwzw(self) -> (A, D, C, D);
	fn xwwx(self) -> (A, D, D, A);
	fn xwwy(self) -> (A, D, D, B);
	fn xwwz(self) -> (A, D, D, C);
	fn xwww(self) -> (A, D, D, D);

	fn yxxx(self) -> (B, A, A, A);
	fn yxxy(self) -> (B, A, A, B);
	fn yxxz(self) -> (B, A, A, C);
	fn yxxw(self) -> (B, A, A, D);
	fn yxyx(self) -> (B, A, B, A);
	fn yxyy(self) -> (B, A, B, B);
	fn yxyz(self) -> (B, A, B, C);
	fn yxyw(self) -> (B, A, B, D);
	fn yxzx(self) -> (B, A, C, A);
	fn yxzy(self) -> (B, A, C, B);
	fn yxzz(self) -> (B, A, C, C);
	fn yxzw(self) -> (B, A, C, D);
	fn yxwx(self) -> (B, A, D, A);
	fn yxwy(self) -> (B, A, D, B);
	fn yxwz(self) -> (B, A, D, C);
	fn yxww(self) -> (B, A, D, D);
	fn yyxx(self) -> (B, B, A, A);
	fn yyxy(self) -> (B, B, A, B);
	fn yyxz(self) -> (B, B, A, C);
	fn yyxw(self) -> (B, B, A, D);
	fn yyyx(self) -> (B, B, B, A);
	fn yyyy(self) -> (B, B, B, B);
	fn yyyz(self) -> (B, B, B, C);
	fn yyyw(self) -> (B, B, B, D);
	fn yyzx(self) -> (B, B, C, A);
	fn yyzy(self) -> (B, B, C, B);
	fn yyzz(self) -> (B, B, C, C);
	fn yyzw(self) -> (B, B, C, D);
	fn yywx(self) -> (B, B, D, A);
	fn yywy(self) -> (B, B, D, B);
	fn yywz(self) -> (B, B, D, C);
	fn yyww(self) -> (B, B, D, D);
	fn yzxx(self) -> (B, C, A, A);
	fn yzxy(self) -> (B, C, A, B);
	fn yzxz(self) -> (B, C, A, C);
	fn yzxw(self) -> (B, C, A, D);
	fn yzyx(self) -> (B, C, B, A);
	fn yzyy(self) -> (B, C, B, B);
	fn yzyz(self) -> (B, C, B, C);
	fn yzyw(self) -> (B, C, B, D);
	fn yzzx(self) -> (B, C, C, A);
	fn yzzy(self) -> (B, C, C, B);
	fn yzzz(self) -> (B, C, C, C);
	fn yzzw(self) -> (B, C, C, D);
	fn yzwx(self) -> (B, C, D, A);
	fn yzwy(self) -> (B, C, D, B);
	fn yzwz(self) -> (B, C, D, C);
	fn yzww(self) -> (B, C, D, D);
	fn ywxx(self) -> (B, D, A, A);
	fn ywxy(self) -> (B, D, A, B);
	fn ywxz(self) -> (B, D, A, C);
	fn ywxw(self) -> (B, D, A, D);
	fn ywyx(self) -> (B, D, B, A);
	fn ywyy(self) -> (B, D, B, B);
	fn ywyz(self) -> (B, D, B, C);
	fn ywyw(self) -> (B, D, B, D);
	fn ywzx(self) -> (B, D, C, A);
	fn ywzy(self) -> (B, D, C, B);
	fn ywzz(self) -> (B, D, C, C);
	fn ywzw(self) -> (B, D, C, D);
	fn ywwx(self) -> (B, D, D, A);
	fn ywwy(self) -> (B, D, D, B);
	fn ywwz(self) -> (B, D, D, C);
	fn ywww(self) -> (B, D, D, D);

	fn zxxx(self) -> (C, A, A, A);
	fn zxxy(self) -> (C, A, A, B);
	fn zxxz(self) -> (C, A, A, C);
	fn zxxw(self) -> (C, A, A, D);
	fn zxyx(self) -> (C, A, B, A);
	fn zxyy(self) -> (C, A, B, B);
	fn zxyz(self) -> (C, A, B, C);
	fn zxyw(self) -> (C, A, B, D);
	fn zxzx(self) -> (C, A, C, A);
	fn zxzy(self) -> (C, A, C, B);
	fn zxzz(self) -> (C, A, C, C);
	fn zxzw(self) -> (C, A, C, D);
	fn zxwx(self) -> (C, A, D, A);
	fn zxwy(self) -> (C, A, D, B);
	fn zxwz(self) -> (C, A, D, C);
	fn zxww(self) -> (C, A, D, D);
	fn zyxx(self) -> (C, B, A, A);
	fn zyxy(self) -> (C, B, A, B);
	fn zyxz(self) -> (C, B, A, C);
	fn zyxw(self) -> (C, B, A, D);
	fn zyyx(self) -> (C, B, B, A);
	fn zyyy(self) -> (C, B, B, B);
	fn zyyz(self) -> (C, B, B, C);
	fn zyyw(self) -> (C, B, B, D);
	fn zyzx(self) -> (C, B, C, A);
	fn zyzy(self) -> (C, B, C, B);
	fn zyzz(self) -> (C, B, C, C);
	fn zyzw(self) -> (C, B, C, D);
	fn zywx(self) -> (C, B, D, A);
	fn zywy(self) -> (C, B, D, B);
	fn zywz(self) -> (C, B, D, C);
	fn zyww(self) -> (C, B, D, D);
	fn zzxx(self) -> (C, C, A, A);
	fn zzxy(self) -> (C, C, A, B);
	fn zzxz(self) -> (C, C, A, C);
	fn zzxw(self) -> (C, C, A, D);
	fn zzyx(self) -> (C, C, B, A);
	fn zzyy(self) -> (C, C, B, B);
	fn zzyz(self) -> (C, C, B, C);
	fn zzyw(self) -> (C, C, B, D);
	fn zzzx(self) -> (C, C, C, A);
	fn zzzy(self) -> (C, C, C, B);
	fn zzzz(self) -> (C, C, C, C);
	fn zzzw(self) -> (C, C, C, D);
	fn zzwx(self) -> (C, C, D, A);
	fn zzwy(self) -> (C, C, D, B);
	fn zzwz(self) -> (C, C, D, C);
	fn zzww(self) -> (C, C, D, D);
	fn zwxx(self) -> (C, D, A, A);
	fn zwxy(self) -> (C, D, A, B);
	fn zwxz(self) -> (C, D, A, C);
	fn zwxw(self) -> (C, D, A, D);
	fn zwyx(self) -> (C, D, B, A);
	fn zwyy(self) -> (C, D, B, B);
	fn zwyz(self) -> (C, D, B, C);
	fn zwyw(self) -> (C, D, B, D);
	fn zwzx(self) -> (C, D, C, A);
	fn zwzy(self) -> (C, D, C, B);
	fn zwzz(self) -> (C, D, C, C);
	fn zwzw(self) -> (C, D, C, D);
	fn zwwx(self) -> (C, D, D, A);
	fn zwwy(self) -> (C, D, D, B);
	fn zwwz(self) -> (C, D, D, C);
	fn zwww(self) -> (C, D, D, D);

	fn wxxx(self) -> (D, A, A, A);
	fn wxxy(self) -> (D, A, A, B);
	fn wxxz(self) -> (D, A, A, C);
	fn wxxw(self) -> (D, A, A, D);
	fn wxyx(self) -> (D, A, B, A);
	fn wxyy(self) -> (D, A, B, B);
	fn wxyz(self) -> (D, A, B, C);
	fn wxyw(self) -> (D, A, B, D);
	fn wxzx(self) -> (D, A, C, A);
	fn wxzy(self) -> (D, A, C, B);
	fn wxzz(self) -> (D, A, C, C);
	fn wxzw(self) -> (D, A, C, D);
	fn wxwx(self) -> (D, A, D, A);
	fn wxwy(self) -> (D, A, D, B);
	fn wxwz(self) -> (D, A, D, C);
	fn wxww(self) -> (D, A, D, D);
	fn wyxx(self) -> (D, B, A, A);
	fn wyxy(self) -> (D, B, A, B);
	fn wyxz(self) -> (D, B, A, C);
	fn wyxw(self) -> (D, B, A, D);
	fn wyyx(self) -> (D, B, B, A);
	fn wyyy(self) -> (D, B, B, B);
	fn wyyz(self) -> (D, B, B, C);
	fn wyyw(self) -> (D, B, B, D);
	fn wyzx(self) -> (D, B, C, A);
	fn wyzy(self) -> (D, B, C, B);
	fn wyzz(self) -> (D, B, C, C);
	fn wyzw(self) -> (D, B, C, D);
	fn wywx(self) -> (D, B, D, A);
	fn wywy(self) -> (D, B, D, B);
	fn wywz(self) -> (D, B, D, C);
	fn wyww(self) -> (D, B, D, D);
	fn wzxx(self) -> (D, C, A, A);
	fn wzxy(self) -> (D, C, A, B);
	fn wzxz(self) -> (D, C, A, C);
	fn wzxw(self) -> (D, C, A, D);
	fn wzyx(self) -> (D, C, B, A);
	fn wzyy(self) -> (D, C, B, B);
	fn wzyz(self) -> (D, C, B, C);
	fn wzyw(self) -> (D, C, B, D);
	fn wzzx(self) -> (D, C, C, A);
	fn wzzy(self) -> (D, C, C, B);
	fn wzzz(self) -> (D, C, C, C);
	fn wzzw(self) -> (D, C, C, D);
	fn wzwx(self) -> (D, C, D, A);
	fn wzwy(self) -> (D, C, D, B);
	fn wzwz(self) -> (D, C, D, C);
	fn wzww(self) -> (D, C, D, D);
	fn wwxx(self) -> (D, D, A, A);
	fn wwxy(self) -> (D, D, A, B);
	fn wwxz(self) -> (D, D, A, C);
	fn wwxw(self) -> (D, D, A, D);
	fn wwyx(self) -> (D, D, B, A);
	fn wwyy(self) -> (D, D, B, B);
	fn wwyz(self) -> (D, D, B, C);
	fn wwyw(self) -> (D, D, B, D);
	fn wwzx(self) -> (D, D, C, A);
	fn wwzy(self) -> (D, D, C, B);
	fn wwzz(self) -> (D, D, C, C);
	fn wwzw(self) -> (D, D, C, D);
	fn wwwx(self) -> (D, D, D, A);
	fn wwwy(self) -> (D, D, D, B);
	fn wwwz(self) -> (D, D, D, C);
	fn wwww(self) -> (D, D, D, D);

	fn r(self) -> A;
	fn g(self) -> B;
	fn b(self) -> C;
	fn a(self) -> D;
	fn rr(self) -> (A, A);
	fn rg(self) -> (A, B);
	fn rb(self) -> (A, C);
	fn ra(self) -> (A, D);
	fn gr(self) -> (B, A);
	fn gg(self) -> (B, B);
	fn gb(self) -> (B, C);
	fn ga(self) -> (B, D);
	fn br(self) -> (C, A);
	fn bg(self) -> (C, B);
	fn bb(self) -> (C, C);
	fn ba(self) -> (C, D);
	fn ar(self) -> (D, A);
	fn ag(self) -> (D, B);
	fn ab(self) -> (D, C);
	fn aa(self) -> (D, D);
	fn rrr(self) -> (A, A, A);
	fn rrg(self) -> (A, A, B);
	fn rrb(self) -> (A, A, C);
	fn rra(self) -> (A, A, D);
	fn rgr(self) -> (A, B, A);
	fn rgg(self) -> (A, B, B);
	fn rgb(self) -> (A, B, C);
	fn rga(self) -> (A, B, D);
	fn rbr(self) -> (A, C, A);
	fn rbg(self) -> (A, C, B);
	fn rbb(self) -> (A, C, C);
	fn rba(self) -> (A, C, D);
	fn rar(self) -> (A, D, A);
	fn rag(self) -> (A, D, B);
	fn rab(self) -> (A, D, C);
	fn raa(self) -> (A, D, D);
	fn grr(self) -> (B, A, A);
	fn grg(self) -> (B, A, B);
	fn grb(self) -> (B, A, C);
	fn gra(self) -> (B, A, D);
	fn ggr(self) -> (B, B, A);
	fn ggg(self) -> (B, B, B);
	fn ggb(self) -> (B, B, C);
	fn gga(self) -> (B, B, D);
	fn gbr(self) -> (B, C, A);
	fn gbg(self) -> (B, C, B);
	fn gbb(self) -> (B, C, C);
	fn gba(self) -> (B, C, D);
	fn gar(self) -> (B, D, A);
	fn gag(self) -> (B, D, B);
	fn gab(self) -> (B, D, C);
	fn gaa(self) -> (B, D, D);
	fn brr(self) -> (C, A, A);
	fn brg(self) -> (C, A, B);
	fn brb(self) -> (C, A, C);
	fn bra(self) -> (C, A, D);
	fn bgr(self) -> (C, B, A);
	fn bgg(self) -> (C, B, B);
	fn bgb(self) -> (C, B, C);
	fn bga(self) -> (C, B, D);
	fn bbr(self) -> (C, C, A);
	fn bbg(self) -> (C, C, B);
	fn bbb(self) -> (C, C, C);
	fn bba(self) -> (C, C, D);
	fn bar(self) -> (C, D, A);
	fn bag(self) -> (C, D, B);
	fn bab(self) -> (C, D, C);
	fn baa(self) -> (C, D, D);
	fn arr(self) -> (D, A, A);
	fn arg(self) -> (D, A, B);
	fn arb(self) -> (D, A, C);
	fn ara(self) -> (D, A, D);
	fn agr(self) -> (D, B, A);
	fn agg(self) -> (D, B, B);
	fn agb(self) -> (D, B, C);
	fn aga(self) -> (D, B, D);
	fn abr(self) -> (D, C, A);
	fn abg(self) -> (D, C, B);
	fn abb(self) -> (D, C, C);
	fn aba(self) -> (D, C, D);
	fn aar(self) -> (D, D, A);
	fn aag(self) -> (D, D, B);
	fn aab(self) -> (D, D, C);
	fn aaa(self) -> (D, D, D);

	fn rrrr(self) -> (A, A, A, A);
	fn rrrg(self) -> (A, A, A, B);
	fn rrrb(self) -> (A, A, A, C);
	fn rrra(self) -> (A, A, A, D);
	fn rrgr(self) -> (A, A, B, A);
	fn rrgg(self) -> (A, A, B, B);
	fn rrgb(self) -> (A, A, B, C);
	fn rrga(self) -> (A, A, B, D);
	fn rrbr(self) -> (A, A, C, A);
	fn rrbg(self) -> (A, A, C, B);
	fn rrbb(self) -> (A, A, C, C);
	fn rrba(self) -> (A, A, C, D);
	fn rrar(self) -> (A, A, D, A);
	fn rrag(self) -> (A, A, D, B);
	fn rrab(self) -> (A, A, D, C);
	fn rraa(self) -> (A, A, D, D);
	fn rgrr(self) -> (A, B, A, A);
	fn rgrg(self) -> (A, B, A, B);
	fn rgrb(self) -> (A, B, A, C);
	fn rgra(self) -> (A, B, A, D);
	fn rggr(self) -> (A, B, B, A);
	fn rggg(self) -> (A, B, B, B);
	fn rggb(self) -> (A, B, B, C);
	fn rgga(self) -> (A, B, B, D);
	fn rgbr(self) -> (A, B, C, A);
	fn rgbg(self) -> (A, B, C, B);
	fn rgbb(self) -> (A, B, C, C);
	fn rgba(self) -> (A, B, C, D);
	fn rgar(self) -> (A, B, D, A);
	fn rgag(self) -> (A, B, D, B);
	fn rgab(self) -> (A, B, D, C);
	fn rgaa(self) -> (A, B, D, D);
	fn rbrr(self) -> (A, C, A, A);
	fn rbrg(self) -> (A, C, A, B);
	fn rbrb(self) -> (A, C, A, C);
	fn rbra(self) -> (A, C, A, D);
	fn rbgr(self) -> (A, C, B, A);
	fn rbgg(self) -> (A, C, B, B);
	fn rbgb(self) -> (A, C, B, C);
	fn rbga(self) -> (A, C, B, D);
	fn rbbr(self) -> (A, C, C, A);
	fn rbbg(self) -> (A, C, C, B);
	fn rbbb(self) -> (A, C, C, C);
	fn rbba(self) -> (A, C, C, D);
	fn rbar(self) -> (A, C, D, A);
	fn rbag(self) -> (A, C, D, B);
	fn rbab(self) -> (A, C, D, C);
	fn rbaa(self) -> (A, C, D, D);
	fn rarr(self) -> (A, D, A, A);
	fn rarg(self) -> (A, D, A, B);
	fn rarb(self) -> (A, D, A, C);
	fn rara(self) -> (A, D, A, D);
	fn ragr(self) -> (A, D, B, A);
	fn ragg(self) -> (A, D, B, B);
	fn ragb(self) -> (A, D, B, C);
	fn raga(self) -> (A, D, B, D);
	fn rabr(self) -> (A, D, C, A);
	fn rabg(self) -> (A, D, C, B);
	fn rabb(self) -> (A, D, C, C);
	fn raba(self) -> (A, D, C, D);
	fn raar(self) -> (A, D, D, A);
	fn raag(self) -> (A, D, D, B);
	fn raab(self) -> (A, D, D, C);
	fn raaa(self) -> (A, D, D, D);

	fn grrr(self) -> (B, A, A, A);
	fn grrg(self) -> (B, A, A, B);
	fn grrb(self) -> (B, A, A, C);
	fn grra(self) -> (B, A, A, D);
	fn grgr(self) -> (B, A, B, A);
	fn grgg(self) -> (B, A, B, B);
	fn grgb(self) -> (B, A, B, C);
	fn grga(self) -> (B, A, B, D);
	fn grbr(self) -> (B, A, C, A);
	fn grbg(self) -> (B, A, C, B);
	fn grbb(self) -> (B, A, C, C);
	fn grba(self) -> (B, A, C, D);
	fn grar(self) -> (B, A, D, A);
	fn grag(self) -> (B, A, D, B);
	fn grab(self) -> (B, A, D, C);
	fn graa(self) -> (B, A, D, D);
	fn ggrr(self) -> (B, B, A, A);
	fn ggrg(self) -> (B, B, A, B);
	fn ggrb(self) -> (B, B, A, C);
	fn ggra(self) -> (B, B, A, D);
	fn gggr(self) -> (B, B, B, A);
	fn gggg(self) -> (B, B, B, B);
	fn gggb(self) -> (B, B, B, C);
	fn ggga(self) -> (B, B, B, D);
	fn ggbr(self) -> (B, B, C, A);
	fn ggbg(self) -> (B, B, C, B);
	fn ggbb(self) -> (B, B, C, C);
	fn ggba(self) -> (B, B, C, D);
	fn ggar(self) -> (B, B, D, A);
	fn ggag(self) -> (B, B, D, B);
	fn ggab(self) -> (B, B, D, C);
	fn ggaa(self) -> (B, B, D, D);
	fn gbrr(self) -> (B, C, A, A);
	fn gbrg(self) -> (B, C, A, B);
	fn gbrb(self) -> (B, C, A, C);
	fn gbra(self) -> (B, C, A, D);
	fn gbgr(self) -> (B, C, B, A);
	fn gbgg(self) -> (B, C, B, B);
	fn gbgb(self) -> (B, C, B, C);
	fn gbga(self) -> (B, C, B, D);
	fn gbbr(self) -> (B, C, C, A);
	fn gbbg(self) -> (B, C, C, B);
	fn gbbb(self) -> (B, C, C, C);
	fn gbba(self) -> (B, C, C, D);
	fn gbar(self) -> (B, C, D, A);
	fn gbag(self) -> (B, C, D, B);
	fn gbab(self) -> (B, C, D, C);
	fn gbaa(self) -> (B, C, D, D);
	fn garr(self) -> (B, D, A, A);
	fn garg(self) -> (B, D, A, B);
	fn garb(self) -> (B, D, A, C);
	fn gara(self) -> (B, D, A, D);
	fn gagr(self) -> (B, D, B, A);
	fn gagg(self) -> (B, D, B, B);
	fn gagb(self) -> (B, D, B, C);
	fn gaga(self) -> (B, D, B, D);
	fn gabr(self) -> (B, D, C, A);
	fn gabg(self) -> (B, D, C, B);
	fn gabb(self) -> (B, D, C, C);
	fn gaba(self) -> (B, D, C, D);
	fn gaar(self) -> (B, D, D, A);
	fn gaag(self) -> (B, D, D, B);
	fn gaab(self) -> (B, D, D, C);
	fn gaaa(self) -> (B, D, D, D);

	fn brrr(self) -> (C, A, A, A);
	fn brrg(self) -> (C, A, A, B);
	fn brrb(self) -> (C, A, A, C);
	fn brra(self) -> (C, A, A, D);
	fn brgr(self) -> (C, A, B, A);
	fn brgg(self) -> (C, A, B, B);
	fn brgb(self) -> (C, A, B, C);
	fn brga(self) -> (C, A, B, D);
	fn brbr(self) -> (C, A, C, A);
	fn brbg(self) -> (C, A, C, B);
	fn brbb(self) -> (C, A, C, C);
	fn brba(self) -> (C, A, C, D);
	fn brar(self) -> (C, A, D, A);
	fn brag(self) -> (C, A, D, B);
	fn brab(self) -> (C, A, D, C);
	fn braa(self) -> (C, A, D, D);
	fn bgrr(self) -> (C, B, A, A);
	fn bgrg(self) -> (C, B, A, B);
	fn bgrb(self) -> (C, B, A, C);
	fn bgra(self) -> (C, B, A, D);
	fn bggr(self) -> (C, B, B, A);
	fn bggg(self) -> (C, B, B, B);
	fn bggb(self) -> (C, B, B, C);
	fn bgga(self) -> (C, B, B, D);
	fn bgbr(self) -> (C, B, C, A);
	fn bgbg(self) -> (C, B, C, B);
	fn bgbb(self) -> (C, B, C, C);
	fn bgba(self) -> (C, B, C, D);
	fn bgar(self) -> (C, B, D, A);
	fn bgag(self) -> (C, B, D, B);
	fn bgab(self) -> (C, B, D, C);
	fn bgaa(self) -> (C, B, D, D);
	fn bbrr(self) -> (C, C, A, A);
	fn bbrg(self) -> (C, C, A, B);
	fn bbrb(self) -> (C, C, A, C);
	fn bbra(self) -> (C, C, A, D);
	fn bbgr(self) -> (C, C, B, A);
	fn bbgg(self) -> (C, C, B, B);
	fn bbgb(self) -> (C, C, B, C);
	fn bbga(self) -> (C, C, B, D);
	fn bbbr(self) -> (C, C, C, A);
	fn bbbg(self) -> (C, C, C, B);
	fn bbbb(self) -> (C, C, C, C);
	fn bbba(self) -> (C, C, C, D);
	fn bbar(self) -> (C, C, D, A);
	fn bbag(self) -> (C, C, D, B);
	fn bbab(self) -> (C, C, D, C);
	fn bbaa(self) -> (C, C, D, D);
	fn barr(self) -> (C, D, A, A);
	fn barg(self) -> (C, D, A, B);
	fn barb(self) -> (C, D, A, C);
	fn bara(self) -> (C, D, A, D);
	fn bagr(self) -> (C, D, B, A);
	fn bagg(self) -> (C, D, B, B);
	fn bagb(self) -> (C, D, B, C);
	fn baga(self) -> (C, D, B, D);
	fn babr(self) -> (C, D, C, A);
	fn babg(self) -> (C, D, C, B);
	fn babb(self) -> (C, D, C, C);
	fn baba(self) -> (C, D, C, D);
	fn baar(self) -> (C, D, D, A);
	fn baag(self) -> (C, D, D, B);
	fn baab(self) -> (C, D, D, C);
	fn baaa(self) -> (C, D, D, D);

	fn arrr(self) -> (D, A, A, A);
	fn arrg(self) -> (D, A, A, B);
	fn arrb(self) -> (D, A, A, C);
	fn arra(self) -> (D, A, A, D);
	fn argr(self) -> (D, A, B, A);
	fn argg(self) -> (D, A, B, B);
	fn argb(self) -> (D, A, B, C);
	fn arga(self) -> (D, A, B, D);
	fn arbr(self) -> (D, A, C, A);
	fn arbg(self) -> (D, A, C, B);
	fn arbb(self) -> (D, A, C, C);
	fn arba(self) -> (D, A, C, D);
	fn arar(self) -> (D, A, D, A);
	fn arag(self) -> (D, A, D, B);
	fn arab(self) -> (D, A, D, C);
	fn araa(self) -> (D, A, D, D);
	fn agrr(self) -> (D, B, A, A);
	fn agrg(self) -> (D, B, A, B);
	fn agrb(self) -> (D, B, A, C);
	fn agra(self) -> (D, B, A, D);
	fn aggr(self) -> (D, B, B, A);
	fn aggg(self) -> (D, B, B, B);
	fn aggb(self) -> (D, B, B, C);
	fn agga(self) -> (D, B, B, D);
	fn agbr(self) -> (D, B, C, A);
	fn agbg(self) -> (D, B, C, B);
	fn agbb(self) -> (D, B, C, C);
	fn agba(self) -> (D, B, C, D);
	fn agar(self) -> (D, B, D, A);
	fn agag(self) -> (D, B, D, B);
	fn agab(self) -> (D, B, D, C);
	fn agaa(self) -> (D, B, D, D);
	fn abrr(self) -> (D, C, A, A);
	fn abrg(self) -> (D, C, A, B);
	fn abrb(self) -> (D, C, A, C);
	fn abra(self) -> (D, C, A, D);
	fn abgr(self) -> (D, C, B, A);
	fn abgg(self) -> (D, C, B, B);
	fn abgb(self) -> (D, C, B, C);
	fn abga(self) -> (D, C, B, D);
	fn abbr(self) -> (D, C, C, A);
	fn abbg(self) -> (D, C, C, B);
	fn abbb(self) -> (D, C, C, C);
	fn abba(self) -> (D, C, C, D);
	fn abar(self) -> (D, C, D, A);
	fn abag(self) -> (D, C, D, B);
	fn abab(self) -> (D, C, D, C);
	fn abaa(self) -> (D, C, D, D);
	fn aarr(self) -> (D, D, A, A);
	fn aarg(self) -> (D, D, A, B);
	fn aarb(self) -> (D, D, A, C);
	fn aara(self) -> (D, D, A, D);
	fn aagr(self) -> (D, D, B, A);
	fn aagg(self) -> (D, D, B, B);
	fn aagb(self) -> (D, D, B, C);
	fn aaga(self) -> (D, D, B, D);
	fn aabr(self) -> (D, D, C, A);
	fn aabg(self) -> (D, D, C, B);
	fn aabb(self) -> (D, D, C, C);
	fn aaba(self) -> (D, D, C, D);
	fn aaar(self) -> (D, D, D, A);
	fn aaag(self) -> (D, D, D, B);
	fn aaab(self) -> (D, D, D, C);
	fn aaaa(self) -> (D, D, D, D);
}

impl<A: Copy, B: Copy> TupleSwizzle2<A, B> for (A, B) {
	fn x(self) -> A { self.0 }
	fn y(self) -> B { self.1 }
	fn xx(self) -> (A, A) { (self.0, self.0) }
	fn xy(self) -> (A, B) { (self.0, self.1) }
	fn yx(self) -> (B, A) { (self.1, self.0) }
	fn yy(self) -> (B, B) { (self.1, self.1) }

	fn r(self) -> A { self.0 }
	fn g(self) -> B { self.1 }
	fn rr(self) -> (A, A) { (self.0, self.0) }
	fn rg(self) -> (A, B) { (self.0, self.1) }
	fn gr(self) -> (B, A) { (self.1, self.0) }
	fn gg(self) -> (B, B) { (self.1, self.1) }
}

impl<A: Copy, B: Copy, C: Copy> TupleSwizzle3<A, B, C> for (A, B, C) {
	fn x(self) -> A { self.0 }
	fn y(self) -> B { self.1 }
	fn z(self) -> C { self.2 }
	fn xx(self) -> (A, A){ (self.0, self.0) }
	fn xy(self) -> (A, B){ (self.0, self.1) }
	fn xz(self) -> (A, C){ (self.0, self.2) }
	fn yx(self) -> (B, A){ (self.1, self.0) }
	fn yy(self) -> (B, B){ (self.1, self.1) }
	fn yz(self) -> (B, C){ (self.1, self.2) }
	fn zx(self) -> (C, A){ (self.2, self.0) }
	fn zy(self) -> (C, B){ (self.2, self.1) }
	fn zz(self) -> (C, C){ (self.2, self.2) }
	fn xxx(self) -> (A, A, A) { (self.0, self.0, self.0) }
	fn xxy(self) -> (A, A, B) { (self.0, self.0, self.1) }
	fn xxz(self) -> (A, A, C) { (self.0, self.0, self.2) }
	fn xyx(self) -> (A, B, A) { (self.0, self.1, self.0) }
	fn xyy(self) -> (A, B, B) { (self.0, self.1, self.1) }
	fn xyz(self) -> (A, B, C) { (self.0, self.1, self.2) }
	fn xzx(self) -> (A, C, A) { (self.0, self.2, self.0) }
	fn xzy(self) -> (A, C, B) { (self.0, self.2, self.1) }
	fn xzz(self) -> (A, C, C) { (self.0, self.2, self.2) }
	fn yxx(self) -> (B, A, A) { (self.1, self.0, self.0) }
	fn yxy(self) -> (B, A, B) { (self.1, self.0, self.1) }
	fn yxz(self) -> (B, A, C) { (self.1, self.0, self.2) }
	fn yyx(self) -> (B, B, A) { (self.1, self.1, self.0) }
	fn yyy(self) -> (B, B, B) { (self.1, self.1, self.1) }
	fn yyz(self) -> (B, B, C) { (self.1, self.1, self.2) }
	fn yzx(self) -> (B, C, A) { (self.1, self.2, self.0) }
	fn yzy(self) -> (B, C, B) { (self.1, self.2, self.1) }
	fn yzz(self) -> (B, C, C) { (self.1, self.2, self.2) }
	fn zxx(self) -> (C, A, A) { (self.2, self.0, self.0) }
	fn zxy(self) -> (C, A, B) { (self.2, self.0, self.1) }
	fn zxz(self) -> (C, A, C) { (self.2, self.0, self.2) }
	fn zyx(self) -> (C, B, A) { (self.2, self.1, self.0) }
	fn zyy(self) -> (C, B, B) { (self.2, self.1, self.1) }
	fn zyz(self) -> (C, B, C) { (self.2, self.1, self.2) }
	fn zzx(self) -> (C, C, A) { (self.2, self.2, self.0) }
	fn zzy(self) -> (C, C, B) { (self.2, self.2, self.1) }
	fn zzz(self) -> (C, C, C) { (self.2, self.2, self.2) }

	fn r(self) -> A { self.0 }
	fn g(self) -> B { self.1 }
	fn b(self) -> C { self.2 }
	fn rr(self) -> (A, A){ (self.0, self.0) }
	fn rg(self) -> (A, B){ (self.0, self.1) }
	fn rb(self) -> (A, C){ (self.0, self.2) }
	fn gr(self) -> (B, A){ (self.1, self.0) }
	fn gg(self) -> (B, B){ (self.1, self.1) }
	fn gb(self) -> (B, C){ (self.1, self.2) }
	fn br(self) -> (C, A){ (self.2, self.0) }
	fn bg(self) -> (C, B){ (self.2, self.1) }
	fn bb(self) -> (C, C){ (self.2, self.2) }
	fn rrr(self) -> (A, A, A) { (self.0, self.0, self.0) }
	fn rrg(self) -> (A, A, B) { (self.0, self.0, self.1) }
	fn rrb(self) -> (A, A, C) { (self.0, self.0, self.2) }
	fn rgr(self) -> (A, B, A) { (self.0, self.1, self.0) }
	fn rgg(self) -> (A, B, B) { (self.0, self.1, self.1) }
	fn rgb(self) -> (A, B, C) { (self.0, self.1, self.2) }
	fn rbr(self) -> (A, C, A) { (self.0, self.2, self.0) }
	fn rbg(self) -> (A, C, B) { (self.0, self.2, self.1) }
	fn rbb(self) -> (A, C, C) { (self.0, self.2, self.2) }
	fn grr(self) -> (B, A, A) { (self.1, self.0, self.0) }
	fn grg(self) -> (B, A, B) { (self.1, self.0, self.1) }
	fn grb(self) -> (B, A, C) { (self.1, self.0, self.2) }
	fn ggr(self) -> (B, B, A) { (self.1, self.1, self.0) }
	fn ggg(self) -> (B, B, B) { (self.1, self.1, self.1) }
	fn ggb(self) -> (B, B, C) { (self.1, self.1, self.2) }
	fn gbr(self) -> (B, C, A) { (self.1, self.2, self.0) }
	fn gbg(self) -> (B, C, B) { (self.1, self.2, self.1) }
	fn gbb(self) -> (B, C, C) { (self.1, self.2, self.2) }
	fn brr(self) -> (C, A, A) { (self.2, self.0, self.0) }
	fn brg(self) -> (C, A, B) { (self.2, self.0, self.1) }
	fn brb(self) -> (C, A, C) { (self.2, self.0, self.2) }
	fn bgr(self) -> (C, B, A) { (self.2, self.1, self.0) }
	fn bgg(self) -> (C, B, B) { (self.2, self.1, self.1) }
	fn bgb(self) -> (C, B, C) { (self.2, self.1, self.2) }
	fn bbr(self) -> (C, C, A) { (self.2, self.2, self.0) }
	fn bbg(self) -> (C, C, B) { (self.2, self.2, self.1) }
	fn bbb(self) -> (C, C, C) { (self.2, self.2, self.2) }
}

impl<A: Copy, B: Copy, C: Copy, D: Copy> TupleSwizzle4<A, B, C, D> for (A, B, C, D) {
	fn x(self) -> A { self.0 }
	fn y(self) -> B { self.1 }
	fn z(self) -> C { self.2 }
	fn w(self) -> D { self.3 }
	fn xx(self) -> (A, A) { (self.0, self.0) }
	fn xy(self) -> (A, B) { (self.0, self.1) }
	fn xz(self) -> (A, C) { (self.0, self.2) }
	fn xw(self) -> (A, D) { (self.0, self.3) }
	fn yx(self) -> (B, A) { (self.1, self.0) }
	fn yy(self) -> (B, B) { (self.1, self.1) }
	fn yz(self) -> (B, C) { (self.1, self.2) }
	fn yw(self) -> (B, D) { (self.1, self.3) }
	fn zx(self) -> (C, A) { (self.2, self.0) }
	fn zy(self) -> (C, B) { (self.2, self.1) }
	fn zz(self) -> (C, C) { (self.2, self.2) }
	fn zw(self) -> (C, D) { (self.2, self.3) }
	fn wx(self) -> (D, A) { (self.3, self.0) }
	fn wy(self) -> (D, B) { (self.3, self.1) }
	fn wz(self) -> (D, C) { (self.3, self.2) }
	fn ww(self) -> (D, D) { (self.3, self.3) }
	fn xxx(self) -> (A, A, A) { (self.0, self.0, self.0) }
	fn xxy(self) -> (A, A, B) { (self.0, self.0, self.1) }
	fn xxz(self) -> (A, A, C) { (self.0, self.0, self.2) }
	fn xxw(self) -> (A, A, D) { (self.0, self.0, self.3) }
	fn xyx(self) -> (A, B, A) { (self.0, self.1, self.0) }
	fn xyy(self) -> (A, B, B) { (self.0, self.1, self.1) }
	fn xyz(self) -> (A, B, C) { (self.0, self.1, self.2) }
	fn xyw(self) -> (A, B, D) { (self.0, self.1, self.3) }
	fn xzx(self) -> (A, C, A) { (self.0, self.2, self.0) }
	fn xzy(self) -> (A, C, B) { (self.0, self.2, self.1) }
	fn xzz(self) -> (A, C, C) { (self.0, self.2, self.2) }
	fn xzw(self) -> (A, C, D) { (self.0, self.2, self.3) }
	fn xwx(self) -> (A, D, A) { (self.0, self.3, self.0) }
	fn xwy(self) -> (A, D, B) { (self.0, self.3, self.1) }
	fn xwz(self) -> (A, D, C) { (self.0, self.3, self.2) }
	fn xww(self) -> (A, D, D) { (self.0, self.3, self.3) }
	fn yxx(self) -> (B, A, A) { (self.1, self.0, self.0) }
	fn yxy(self) -> (B, A, B) { (self.1, self.0, self.1) }
	fn yxz(self) -> (B, A, C) { (self.1, self.0, self.2) }
	fn yxw(self) -> (B, A, D) { (self.1, self.0, self.3) }
	fn yyx(self) -> (B, B, A) { (self.1, self.1, self.0) }
	fn yyy(self) -> (B, B, B) { (self.1, self.1, self.1) }
	fn yyz(self) -> (B, B, C) { (self.1, self.1, self.2) }
	fn yyw(self) -> (B, B, D) { (self.1, self.1, self.3) }
	fn yzx(self) -> (B, C, A) { (self.1, self.2, self.0) }
	fn yzy(self) -> (B, C, B) { (self.1, self.2, self.1) }
	fn yzz(self) -> (B, C, C) { (self.1, self.2, self.2) }
	fn yzw(self) -> (B, C, D) { (self.1, self.2, self.3) }
	fn ywx(self) -> (B, D, A) { (self.1, self.3, self.0) }
	fn ywy(self) -> (B, D, B) { (self.1, self.3, self.1) }
	fn ywz(self) -> (B, D, C) { (self.1, self.3, self.2) }
	fn yww(self) -> (B, D, D) { (self.1, self.3, self.3) }
	fn zxx(self) -> (C, A, A) { (self.2, self.0, self.0) }
	fn zxy(self) -> (C, A, B) { (self.2, self.0, self.1) }
	fn zxz(self) -> (C, A, C) { (self.2, self.0, self.2) }
	fn zxw(self) -> (C, A, D) { (self.2, self.0, self.3) }
	fn zyx(self) -> (C, B, A) { (self.2, self.1, self.0) }
	fn zyy(self) -> (C, B, B) { (self.2, self.1, self.1) }
	fn zyz(self) -> (C, B, C) { (self.2, self.1, self.2) }
	fn zyw(self) -> (C, B, D) { (self.2, self.1, self.3) }
	fn zzx(self) -> (C, C, A) { (self.2, self.2, self.0) }
	fn zzy(self) -> (C, C, B) { (self.2, self.2, self.1) }
	fn zzz(self) -> (C, C, C) { (self.2, self.2, self.2) }
	fn zzw(self) -> (C, C, D) { (self.2, self.2, self.3) }
	fn zwx(self) -> (C, D, A) { (self.2, self.3, self.0) }
	fn zwy(self) -> (C, D, B) { (self.2, self.3, self.1) }
	fn zwz(self) -> (C, D, C) { (self.2, self.3, self.2) }
	fn zww(self) -> (C, D, D) { (self.2, self.3, self.3) }
	fn wxx(self) -> (D, A, A) { (self.3, self.0, self.0) }
	fn wxy(self) -> (D, A, B) { (self.3, self.0, self.1) }
	fn wxz(self) -> (D, A, C) { (self.3, self.0, self.2) }
	fn wxw(self) -> (D, A, D) { (self.3, self.0, self.3) }
	fn wyx(self) -> (D, B, A) { (self.3, self.1, self.0) }
	fn wyy(self) -> (D, B, B) { (self.3, self.1, self.1) }
	fn wyz(self) -> (D, B, C) { (self.3, self.1, self.2) }
	fn wyw(self) -> (D, B, D) { (self.3, self.1, self.3) }
	fn wzx(self) -> (D, C, A) { (self.3, self.2, self.0) }
	fn wzy(self) -> (D, C, B) { (self.3, self.2, self.1) }
	fn wzz(self) -> (D, C, C) { (self.3, self.2, self.2) }
	fn wzw(self) -> (D, C, D) { (self.3, self.2, self.3) }
	fn wwx(self) -> (D, D, A) { (self.3, self.3, self.0) }
	fn wwy(self) -> (D, D, B) { (self.3, self.3, self.1) }
	fn wwz(self) -> (D, D, C) { (self.3, self.3, self.2) }
	fn www(self) -> (D, D, D) { (self.3, self.3, self.3) }

	fn xxxx(self) -> (A, A, A, A) { (self.0, self.0, self.0, self.0) }
	fn xxxy(self) -> (A, A, A, B) { (self.0, self.0, self.0, self.1) }
	fn xxxz(self) -> (A, A, A, C) { (self.0, self.0, self.0, self.2) }
	fn xxxw(self) -> (A, A, A, D) { (self.0, self.0, self.0, self.3) }
	fn xxyx(self) -> (A, A, B, A) { (self.0, self.0, self.1, self.0) }
	fn xxyy(self) -> (A, A, B, B) { (self.0, self.0, self.1, self.1) }
	fn xxyz(self) -> (A, A, B, C) { (self.0, self.0, self.1, self.2) }
	fn xxyw(self) -> (A, A, B, D) { (self.0, self.0, self.1, self.3) }
	fn xxzx(self) -> (A, A, C, A) { (self.0, self.0, self.2, self.0) }
	fn xxzy(self) -> (A, A, C, B) { (self.0, self.0, self.2, self.1) }
	fn xxzz(self) -> (A, A, C, C) { (self.0, self.0, self.2, self.2) }
	fn xxzw(self) -> (A, A, C, D) { (self.0, self.0, self.2, self.3) }
	fn xxwx(self) -> (A, A, D, A) { (self.0, self.0, self.3, self.0) }
	fn xxwy(self) -> (A, A, D, B) { (self.0, self.0, self.3, self.1) }
	fn xxwz(self) -> (A, A, D, C) { (self.0, self.0, self.3, self.2) }
	fn xxww(self) -> (A, A, D, D) { (self.0, self.0, self.3, self.3) }
	fn xyxx(self) -> (A, B, A, A) { (self.0, self.1, self.0, self.0) }
	fn xyxy(self) -> (A, B, A, B) { (self.0, self.1, self.0, self.1) }
	fn xyxz(self) -> (A, B, A, C) { (self.0, self.1, self.0, self.2) }
	fn xyxw(self) -> (A, B, A, D) { (self.0, self.1, self.0, self.3) }
	fn xyyx(self) -> (A, B, B, A) { (self.0, self.1, self.1, self.0) }
	fn xyyy(self) -> (A, B, B, B) { (self.0, self.1, self.1, self.1) }
	fn xyyz(self) -> (A, B, B, C) { (self.0, self.1, self.1, self.2) }
	fn xyyw(self) -> (A, B, B, D) { (self.0, self.1, self.1, self.3) }
	fn xyzx(self) -> (A, B, C, A) { (self.0, self.1, self.2, self.0) }
	fn xyzy(self) -> (A, B, C, B) { (self.0, self.1, self.2, self.1) }
	fn xyzz(self) -> (A, B, C, C) { (self.0, self.1, self.2, self.2) }
	fn xyzw(self) -> (A, B, C, D) { (self.0, self.1, self.2, self.3) }
	fn xywx(self) -> (A, B, D, A) { (self.0, self.1, self.3, self.0) }
	fn xywy(self) -> (A, B, D, B) { (self.0, self.1, self.3, self.1) }
	fn xywz(self) -> (A, B, D, C) { (self.0, self.1, self.3, self.2) }
	fn xyww(self) -> (A, B, D, D) { (self.0, self.1, self.3, self.3) }
	fn xzxx(self) -> (A, C, A, A) { (self.0, self.2, self.0, self.0) }
	fn xzxy(self) -> (A, C, A, B) { (self.0, self.2, self.0, self.1) }
	fn xzxz(self) -> (A, C, A, C) { (self.0, self.2, self.0, self.2) }
	fn xzxw(self) -> (A, C, A, D) { (self.0, self.2, self.0, self.3) }
	fn xzyx(self) -> (A, C, B, A) { (self.0, self.2, self.1, self.0) }
	fn xzyy(self) -> (A, C, B, B) { (self.0, self.2, self.1, self.1) }
	fn xzyz(self) -> (A, C, B, C) { (self.0, self.2, self.1, self.2) }
	fn xzyw(self) -> (A, C, B, D) { (self.0, self.2, self.1, self.3) }
	fn xzzx(self) -> (A, C, C, A) { (self.0, self.2, self.2, self.0) }
	fn xzzy(self) -> (A, C, C, B) { (self.0, self.2, self.2, self.1) }
	fn xzzz(self) -> (A, C, C, C) { (self.0, self.2, self.2, self.2) }
	fn xzzw(self) -> (A, C, C, D) { (self.0, self.2, self.2, self.3) }
	fn xzwx(self) -> (A, C, D, A) { (self.0, self.2, self.3, self.0) }
	fn xzwy(self) -> (A, C, D, B) { (self.0, self.2, self.3, self.1) }
	fn xzwz(self) -> (A, C, D, C) { (self.0, self.2, self.3, self.2) }
	fn xzww(self) -> (A, C, D, D) { (self.0, self.2, self.3, self.3) }
	fn xwxx(self) -> (A, D, A, A) { (self.0, self.3, self.0, self.0) }
	fn xwxy(self) -> (A, D, A, B) { (self.0, self.3, self.0, self.1) }
	fn xwxz(self) -> (A, D, A, C) { (self.0, self.3, self.0, self.2) }
	fn xwxw(self) -> (A, D, A, D) { (self.0, self.3, self.0, self.3) }
	fn xwyx(self) -> (A, D, B, A) { (self.0, self.3, self.1, self.0) }
	fn xwyy(self) -> (A, D, B, B) { (self.0, self.3, self.1, self.1) }
	fn xwyz(self) -> (A, D, B, C) { (self.0, self.3, self.1, self.2) }
	fn xwyw(self) -> (A, D, B, D) { (self.0, self.3, self.1, self.3) }
	fn xwzx(self) -> (A, D, C, A) { (self.0, self.3, self.2, self.0) }
	fn xwzy(self) -> (A, D, C, B) { (self.0, self.3, self.2, self.1) }
	fn xwzz(self) -> (A, D, C, C) { (self.0, self.3, self.2, self.2) }
	fn xwzw(self) -> (A, D, C, D) { (self.0, self.3, self.2, self.3) }
	fn xwwx(self) -> (A, D, D, A) { (self.0, self.3, self.3, self.0) }
	fn xwwy(self) -> (A, D, D, B) { (self.0, self.3, self.3, self.1) }
	fn xwwz(self) -> (A, D, D, C) { (self.0, self.3, self.3, self.2) }
	fn xwww(self) -> (A, D, D, D) { (self.0, self.3, self.3, self.3) }

	fn yxxx(self) -> (B, A, A, A) { (self.1, self.0, self.0, self.0) }
	fn yxxy(self) -> (B, A, A, B) { (self.1, self.0, self.0, self.1) }
	fn yxxz(self) -> (B, A, A, C) { (self.1, self.0, self.0, self.2) }
	fn yxxw(self) -> (B, A, A, D) { (self.1, self.0, self.0, self.3) }
	fn yxyx(self) -> (B, A, B, A) { (self.1, self.0, self.1, self.0) }
	fn yxyy(self) -> (B, A, B, B) { (self.1, self.0, self.1, self.1) }
	fn yxyz(self) -> (B, A, B, C) { (self.1, self.0, self.1, self.2) }
	fn yxyw(self) -> (B, A, B, D) { (self.1, self.0, self.1, self.3) }
	fn yxzx(self) -> (B, A, C, A) { (self.1, self.0, self.2, self.0) }
	fn yxzy(self) -> (B, A, C, B) { (self.1, self.0, self.2, self.1) }
	fn yxzz(self) -> (B, A, C, C) { (self.1, self.0, self.2, self.2) }
	fn yxzw(self) -> (B, A, C, D) { (self.1, self.0, self.2, self.3) }
	fn yxwx(self) -> (B, A, D, A) { (self.1, self.0, self.3, self.0) }
	fn yxwy(self) -> (B, A, D, B) { (self.1, self.0, self.3, self.1) }
	fn yxwz(self) -> (B, A, D, C) { (self.1, self.0, self.3, self.2) }
	fn yxww(self) -> (B, A, D, D) { (self.1, self.0, self.3, self.3) }
	fn yyxx(self) -> (B, B, A, A) { (self.1, self.1, self.0, self.0) }
	fn yyxy(self) -> (B, B, A, B) { (self.1, self.1, self.0, self.1) }
	fn yyxz(self) -> (B, B, A, C) { (self.1, self.1, self.0, self.2) }
	fn yyxw(self) -> (B, B, A, D) { (self.1, self.1, self.0, self.3) }
	fn yyyx(self) -> (B, B, B, A) { (self.1, self.1, self.1, self.0) }
	fn yyyy(self) -> (B, B, B, B) { (self.1, self.1, self.1, self.1) }
	fn yyyz(self) -> (B, B, B, C) { (self.1, self.1, self.1, self.2) }
	fn yyyw(self) -> (B, B, B, D) { (self.1, self.1, self.1, self.3) }
	fn yyzx(self) -> (B, B, C, A) { (self.1, self.1, self.2, self.0) }
	fn yyzy(self) -> (B, B, C, B) { (self.1, self.1, self.2, self.1) }
	fn yyzz(self) -> (B, B, C, C) { (self.1, self.1, self.2, self.2) }
	fn yyzw(self) -> (B, B, C, D) { (self.1, self.1, self.2, self.3) }
	fn yywx(self) -> (B, B, D, A) { (self.1, self.1, self.3, self.0) }
	fn yywy(self) -> (B, B, D, B) { (self.1, self.1, self.3, self.1) }
	fn yywz(self) -> (B, B, D, C) { (self.1, self.1, self.3, self.2) }
	fn yyww(self) -> (B, B, D, D) { (self.1, self.1, self.3, self.3) }
	fn yzxx(self) -> (B, C, A, A) { (self.1, self.2, self.0, self.0) }
	fn yzxy(self) -> (B, C, A, B) { (self.1, self.2, self.0, self.1) }
	fn yzxz(self) -> (B, C, A, C) { (self.1, self.2, self.0, self.2) }
	fn yzxw(self) -> (B, C, A, D) { (self.1, self.2, self.0, self.3) }
	fn yzyx(self) -> (B, C, B, A) { (self.1, self.2, self.1, self.0) }
	fn yzyy(self) -> (B, C, B, B) { (self.1, self.2, self.1, self.1) }
	fn yzyz(self) -> (B, C, B, C) { (self.1, self.2, self.1, self.2) }
	fn yzyw(self) -> (B, C, B, D) { (self.1, self.2, self.1, self.3) }
	fn yzzx(self) -> (B, C, C, A) { (self.1, self.2, self.2, self.0) }
	fn yzzy(self) -> (B, C, C, B) { (self.1, self.2, self.2, self.1) }
	fn yzzz(self) -> (B, C, C, C) { (self.1, self.2, self.2, self.2) }
	fn yzzw(self) -> (B, C, C, D) { (self.1, self.2, self.2, self.3) }
	fn yzwx(self) -> (B, C, D, A) { (self.1, self.2, self.3, self.0) }
	fn yzwy(self) -> (B, C, D, B) { (self.1, self.2, self.3, self.1) }
	fn yzwz(self) -> (B, C, D, C) { (self.1, self.2, self.3, self.2) }
	fn yzww(self) -> (B, C, D, D) { (self.1, self.2, self.3, self.3) }
	fn ywxx(self) -> (B, D, A, A) { (self.1, self.3, self.0, self.0) }
	fn ywxy(self) -> (B, D, A, B) { (self.1, self.3, self.0, self.1) }
	fn ywxz(self) -> (B, D, A, C) { (self.1, self.3, self.0, self.2) }
	fn ywxw(self) -> (B, D, A, D) { (self.1, self.3, self.0, self.3) }
	fn ywyx(self) -> (B, D, B, A) { (self.1, self.3, self.1, self.0) }
	fn ywyy(self) -> (B, D, B, B) { (self.1, self.3, self.1, self.1) }
	fn ywyz(self) -> (B, D, B, C) { (self.1, self.3, self.1, self.2) }
	fn ywyw(self) -> (B, D, B, D) { (self.1, self.3, self.1, self.3) }
	fn ywzx(self) -> (B, D, C, A) { (self.1, self.3, self.2, self.0) }
	fn ywzy(self) -> (B, D, C, B) { (self.1, self.3, self.2, self.1) }
	fn ywzz(self) -> (B, D, C, C) { (self.1, self.3, self.2, self.2) }
	fn ywzw(self) -> (B, D, C, D) { (self.1, self.3, self.2, self.3) }
	fn ywwx(self) -> (B, D, D, A) { (self.1, self.3, self.3, self.0) }
	fn ywwy(self) -> (B, D, D, B) { (self.1, self.3, self.3, self.1) }
	fn ywwz(self) -> (B, D, D, C) { (self.1, self.3, self.3, self.2) }
	fn ywww(self) -> (B, D, D, D) { (self.1, self.3, self.3, self.3) }

	fn zxxx(self) -> (C, A, A, A) { (self.2, self.0, self.0, self.0) }
	fn zxxy(self) -> (C, A, A, B) { (self.2, self.0, self.0, self.1) }
	fn zxxz(self) -> (C, A, A, C) { (self.2, self.0, self.0, self.2) }
	fn zxxw(self) -> (C, A, A, D) { (self.2, self.0, self.0, self.3) }
	fn zxyx(self) -> (C, A, B, A) { (self.2, self.0, self.1, self.0) }
	fn zxyy(self) -> (C, A, B, B) { (self.2, self.0, self.1, self.1) }
	fn zxyz(self) -> (C, A, B, C) { (self.2, self.0, self.1, self.2) }
	fn zxyw(self) -> (C, A, B, D) { (self.2, self.0, self.1, self.3) }
	fn zxzx(self) -> (C, A, C, A) { (self.2, self.0, self.2, self.0) }
	fn zxzy(self) -> (C, A, C, B) { (self.2, self.0, self.2, self.1) }
	fn zxzz(self) -> (C, A, C, C) { (self.2, self.0, self.2, self.2) }
	fn zxzw(self) -> (C, A, C, D) { (self.2, self.0, self.2, self.3) }
	fn zxwx(self) -> (C, A, D, A) { (self.2, self.0, self.3, self.0) }
	fn zxwy(self) -> (C, A, D, B) { (self.2, self.0, self.3, self.1) }
	fn zxwz(self) -> (C, A, D, C) { (self.2, self.0, self.3, self.2) }
	fn zxww(self) -> (C, A, D, D) { (self.2, self.0, self.3, self.3) }
	fn zyxx(self) -> (C, B, A, A) { (self.2, self.1, self.0, self.0) }
	fn zyxy(self) -> (C, B, A, B) { (self.2, self.1, self.0, self.1) }
	fn zyxz(self) -> (C, B, A, C) { (self.2, self.1, self.0, self.2) }
	fn zyxw(self) -> (C, B, A, D) { (self.2, self.1, self.0, self.3) }
	fn zyyx(self) -> (C, B, B, A) { (self.2, self.1, self.1, self.0) }
	fn zyyy(self) -> (C, B, B, B) { (self.2, self.1, self.1, self.1) }
	fn zyyz(self) -> (C, B, B, C) { (self.2, self.1, self.1, self.2) }
	fn zyyw(self) -> (C, B, B, D) { (self.2, self.1, self.1, self.3) }
	fn zyzx(self) -> (C, B, C, A) { (self.2, self.1, self.2, self.0) }
	fn zyzy(self) -> (C, B, C, B) { (self.2, self.1, self.2, self.1) }
	fn zyzz(self) -> (C, B, C, C) { (self.2, self.1, self.2, self.2) }
	fn zyzw(self) -> (C, B, C, D) { (self.2, self.1, self.2, self.3) }
	fn zywx(self) -> (C, B, D, A) { (self.2, self.1, self.3, self.0) }
	fn zywy(self) -> (C, B, D, B) { (self.2, self.1, self.3, self.1) }
	fn zywz(self) -> (C, B, D, C) { (self.2, self.1, self.3, self.2) }
	fn zyww(self) -> (C, B, D, D) { (self.2, self.1, self.3, self.3) }
	fn zzxx(self) -> (C, C, A, A) { (self.2, self.2, self.0, self.0) }
	fn zzxy(self) -> (C, C, A, B) { (self.2, self.2, self.0, self.1) }
	fn zzxz(self) -> (C, C, A, C) { (self.2, self.2, self.0, self.2) }
	fn zzxw(self) -> (C, C, A, D) { (self.2, self.2, self.0, self.3) }
	fn zzyx(self) -> (C, C, B, A) { (self.2, self.2, self.1, self.0) }
	fn zzyy(self) -> (C, C, B, B) { (self.2, self.2, self.1, self.1) }
	fn zzyz(self) -> (C, C, B, C) { (self.2, self.2, self.1, self.2) }
	fn zzyw(self) -> (C, C, B, D) { (self.2, self.2, self.1, self.3) }
	fn zzzx(self) -> (C, C, C, A) { (self.2, self.2, self.2, self.0) }
	fn zzzy(self) -> (C, C, C, B) { (self.2, self.2, self.2, self.1) }
	fn zzzz(self) -> (C, C, C, C) { (self.2, self.2, self.2, self.2) }
	fn zzzw(self) -> (C, C, C, D) { (self.2, self.2, self.2, self.3) }
	fn zzwx(self) -> (C, C, D, A) { (self.2, self.2, self.3, self.0) }
	fn zzwy(self) -> (C, C, D, B) { (self.2, self.2, self.3, self.1) }
	fn zzwz(self) -> (C, C, D, C) { (self.2, self.2, self.3, self.2) }
	fn zzww(self) -> (C, C, D, D) { (self.2, self.2, self.3, self.3) }
	fn zwxx(self) -> (C, D, A, A) { (self.2, self.3, self.0, self.0) }
	fn zwxy(self) -> (C, D, A, B) { (self.2, self.3, self.0, self.1) }
	fn zwxz(self) -> (C, D, A, C) { (self.2, self.3, self.0, self.2) }
	fn zwxw(self) -> (C, D, A, D) { (self.2, self.3, self.0, self.3) }
	fn zwyx(self) -> (C, D, B, A) { (self.2, self.3, self.1, self.0) }
	fn zwyy(self) -> (C, D, B, B) { (self.2, self.3, self.1, self.1) }
	fn zwyz(self) -> (C, D, B, C) { (self.2, self.3, self.1, self.2) }
	fn zwyw(self) -> (C, D, B, D) { (self.2, self.3, self.1, self.3) }
	fn zwzx(self) -> (C, D, C, A) { (self.2, self.3, self.2, self.0) }
	fn zwzy(self) -> (C, D, C, B) { (self.2, self.3, self.2, self.1) }
	fn zwzz(self) -> (C, D, C, C) { (self.2, self.3, self.2, self.2) }
	fn zwzw(self) -> (C, D, C, D) { (self.2, self.3, self.2, self.3) }
	fn zwwx(self) -> (C, D, D, A) { (self.2, self.3, self.3, self.0) }
	fn zwwy(self) -> (C, D, D, B) { (self.2, self.3, self.3, self.1) }
	fn zwwz(self) -> (C, D, D, C) { (self.2, self.3, self.3, self.2) }
	fn zwww(self) -> (C, D, D, D) { (self.2, self.3, self.3, self.3) }

	fn wxxx(self) -> (D, A, A, A) { (self.3, self.0, self.0, self.0) }
	fn wxxy(self) -> (D, A, A, B) { (self.3, self.0, self.0, self.1) }
	fn wxxz(self) -> (D, A, A, C) { (self.3, self.0, self.0, self.2) }
	fn wxxw(self) -> (D, A, A, D) { (self.3, self.0, self.0, self.3) }
	fn wxyx(self) -> (D, A, B, A) { (self.3, self.0, self.1, self.0) }
	fn wxyy(self) -> (D, A, B, B) { (self.3, self.0, self.1, self.1) }
	fn wxyz(self) -> (D, A, B, C) { (self.3, self.0, self.1, self.2) }
	fn wxyw(self) -> (D, A, B, D) { (self.3, self.0, self.1, self.3) }
	fn wxzx(self) -> (D, A, C, A) { (self.3, self.0, self.2, self.0) }
	fn wxzy(self) -> (D, A, C, B) { (self.3, self.0, self.2, self.1) }
	fn wxzz(self) -> (D, A, C, C) { (self.3, self.0, self.2, self.2) }
	fn wxzw(self) -> (D, A, C, D) { (self.3, self.0, self.2, self.3) }
	fn wxwx(self) -> (D, A, D, A) { (self.3, self.0, self.3, self.0) }
	fn wxwy(self) -> (D, A, D, B) { (self.3, self.0, self.3, self.1) }
	fn wxwz(self) -> (D, A, D, C) { (self.3, self.0, self.3, self.2) }
	fn wxww(self) -> (D, A, D, D) { (self.3, self.0, self.3, self.3) }
	fn wyxx(self) -> (D, B, A, A) { (self.3, self.1, self.0, self.0) }
	fn wyxy(self) -> (D, B, A, B) { (self.3, self.1, self.0, self.1) }
	fn wyxz(self) -> (D, B, A, C) { (self.3, self.1, self.0, self.2) }
	fn wyxw(self) -> (D, B, A, D) { (self.3, self.1, self.0, self.3) }
	fn wyyx(self) -> (D, B, B, A) { (self.3, self.1, self.1, self.0) }
	fn wyyy(self) -> (D, B, B, B) { (self.3, self.1, self.1, self.1) }
	fn wyyz(self) -> (D, B, B, C) { (self.3, self.1, self.1, self.2) }
	fn wyyw(self) -> (D, B, B, D) { (self.3, self.1, self.1, self.3) }
	fn wyzx(self) -> (D, B, C, A) { (self.3, self.1, self.2, self.0) }
	fn wyzy(self) -> (D, B, C, B) { (self.3, self.1, self.2, self.1) }
	fn wyzz(self) -> (D, B, C, C) { (self.3, self.1, self.2, self.2) }
	fn wyzw(self) -> (D, B, C, D) { (self.3, self.1, self.2, self.3) }
	fn wywx(self) -> (D, B, D, A) { (self.3, self.1, self.3, self.0) }
	fn wywy(self) -> (D, B, D, B) { (self.3, self.1, self.3, self.1) }
	fn wywz(self) -> (D, B, D, C) { (self.3, self.1, self.3, self.2) }
	fn wyww(self) -> (D, B, D, D) { (self.3, self.1, self.3, self.3) }
	fn wzxx(self) -> (D, C, A, A) { (self.3, self.2, self.0, self.0) }
	fn wzxy(self) -> (D, C, A, B) { (self.3, self.2, self.0, self.1) }
	fn wzxz(self) -> (D, C, A, C) { (self.3, self.2, self.0, self.2) }
	fn wzxw(self) -> (D, C, A, D) { (self.3, self.2, self.0, self.3) }
	fn wzyx(self) -> (D, C, B, A) { (self.3, self.2, self.1, self.0) }
	fn wzyy(self) -> (D, C, B, B) { (self.3, self.2, self.1, self.1) }
	fn wzyz(self) -> (D, C, B, C) { (self.3, self.2, self.1, self.2) }
	fn wzyw(self) -> (D, C, B, D) { (self.3, self.2, self.1, self.3) }
	fn wzzx(self) -> (D, C, C, A) { (self.3, self.2, self.2, self.0) }
	fn wzzy(self) -> (D, C, C, B) { (self.3, self.2, self.2, self.1) }
	fn wzzz(self) -> (D, C, C, C) { (self.3, self.2, self.2, self.2) }
	fn wzzw(self) -> (D, C, C, D) { (self.3, self.2, self.2, self.3) }
	fn wzwx(self) -> (D, C, D, A) { (self.3, self.2, self.3, self.0) }
	fn wzwy(self) -> (D, C, D, B) { (self.3, self.2, self.3, self.1) }
	fn wzwz(self) -> (D, C, D, C) { (self.3, self.2, self.3, self.2) }
	fn wzww(self) -> (D, C, D, D) { (self.3, self.2, self.3, self.3) }
	fn wwxx(self) -> (D, D, A, A) { (self.3, self.3, self.0, self.0) }
	fn wwxy(self) -> (D, D, A, B) { (self.3, self.3, self.0, self.1) }
	fn wwxz(self) -> (D, D, A, C) { (self.3, self.3, self.0, self.2) }
	fn wwxw(self) -> (D, D, A, D) { (self.3, self.3, self.0, self.3) }
	fn wwyx(self) -> (D, D, B, A) { (self.3, self.3, self.1, self.0) }
	fn wwyy(self) -> (D, D, B, B) { (self.3, self.3, self.1, self.1) }
	fn wwyz(self) -> (D, D, B, C) { (self.3, self.3, self.1, self.2) }
	fn wwyw(self) -> (D, D, B, D) { (self.3, self.3, self.1, self.3) }
	fn wwzx(self) -> (D, D, C, A) { (self.3, self.3, self.2, self.0) }
	fn wwzy(self) -> (D, D, C, B) { (self.3, self.3, self.2, self.1) }
	fn wwzz(self) -> (D, D, C, C) { (self.3, self.3, self.2, self.2) }
	fn wwzw(self) -> (D, D, C, D) { (self.3, self.3, self.2, self.3) }
	fn wwwx(self) -> (D, D, D, A) { (self.3, self.3, self.3, self.0) }
	fn wwwy(self) -> (D, D, D, B) { (self.3, self.3, self.3, self.1) }
	fn wwwz(self) -> (D, D, D, C) { (self.3, self.3, self.3, self.2) }
	fn wwww(self) -> (D, D, D, D) { (self.3, self.3, self.3, self.3) }

	fn r(self) -> A { self.0 }
	fn g(self) -> B { self.1 }
	fn b(self) -> C { self.2 }
	fn a(self) -> D { self.3 }
	fn rr(self) -> (A, A) { (self.0, self.0) }
	fn rg(self) -> (A, B) { (self.0, self.1) }
	fn rb(self) -> (A, C) { (self.0, self.2) }
	fn ra(self) -> (A, D) { (self.0, self.3) }
	fn gr(self) -> (B, A) { (self.1, self.0) }
	fn gg(self) -> (B, B) { (self.1, self.1) }
	fn gb(self) -> (B, C) { (self.1, self.2) }
	fn ga(self) -> (B, D) { (self.1, self.3) }
	fn br(self) -> (C, A) { (self.2, self.0) }
	fn bg(self) -> (C, B) { (self.2, self.1) }
	fn bb(self) -> (C, C) { (self.2, self.2) }
	fn ba(self) -> (C, D) { (self.2, self.3) }
	fn ar(self) -> (D, A) { (self.3, self.0) }
	fn ag(self) -> (D, B) { (self.3, self.1) }
	fn ab(self) -> (D, C) { (self.3, self.2) }
	fn aa(self) -> (D, D) { (self.3, self.3) }
	fn rrr(self) -> (A, A, A) { (self.0, self.0, self.0) }
	fn rrg(self) -> (A, A, B) { (self.0, self.0, self.1) }
	fn rrb(self) -> (A, A, C) { (self.0, self.0, self.2) }
	fn rra(self) -> (A, A, D) { (self.0, self.0, self.3) }
	fn rgr(self) -> (A, B, A) { (self.0, self.1, self.0) }
	fn rgg(self) -> (A, B, B) { (self.0, self.1, self.1) }
	fn rgb(self) -> (A, B, C) { (self.0, self.1, self.2) }
	fn rga(self) -> (A, B, D) { (self.0, self.1, self.3) }
	fn rbr(self) -> (A, C, A) { (self.0, self.2, self.0) }
	fn rbg(self) -> (A, C, B) { (self.0, self.2, self.1) }
	fn rbb(self) -> (A, C, C) { (self.0, self.2, self.2) }
	fn rba(self) -> (A, C, D) { (self.0, self.2, self.3) }
	fn rar(self) -> (A, D, A) { (self.0, self.3, self.0) }
	fn rag(self) -> (A, D, B) { (self.0, self.3, self.1) }
	fn rab(self) -> (A, D, C) { (self.0, self.3, self.2) }
	fn raa(self) -> (A, D, D) { (self.0, self.3, self.3) }
	fn grr(self) -> (B, A, A) { (self.1, self.0, self.0) }
	fn grg(self) -> (B, A, B) { (self.1, self.0, self.1) }
	fn grb(self) -> (B, A, C) { (self.1, self.0, self.2) }
	fn gra(self) -> (B, A, D) { (self.1, self.0, self.3) }
	fn ggr(self) -> (B, B, A) { (self.1, self.1, self.0) }
	fn ggg(self) -> (B, B, B) { (self.1, self.1, self.1) }
	fn ggb(self) -> (B, B, C) { (self.1, self.1, self.2) }
	fn gga(self) -> (B, B, D) { (self.1, self.1, self.3) }
	fn gbr(self) -> (B, C, A) { (self.1, self.2, self.0) }
	fn gbg(self) -> (B, C, B) { (self.1, self.2, self.1) }
	fn gbb(self) -> (B, C, C) { (self.1, self.2, self.2) }
	fn gba(self) -> (B, C, D) { (self.1, self.2, self.3) }
	fn gar(self) -> (B, D, A) { (self.1, self.3, self.0) }
	fn gag(self) -> (B, D, B) { (self.1, self.3, self.1) }
	fn gab(self) -> (B, D, C) { (self.1, self.3, self.2) }
	fn gaa(self) -> (B, D, D) { (self.1, self.3, self.3) }
	fn brr(self) -> (C, A, A) { (self.2, self.0, self.0) }
	fn brg(self) -> (C, A, B) { (self.2, self.0, self.1) }
	fn brb(self) -> (C, A, C) { (self.2, self.0, self.2) }
	fn bra(self) -> (C, A, D) { (self.2, self.0, self.3) }
	fn bgr(self) -> (C, B, A) { (self.2, self.1, self.0) }
	fn bgg(self) -> (C, B, B) { (self.2, self.1, self.1) }
	fn bgb(self) -> (C, B, C) { (self.2, self.1, self.2) }
	fn bga(self) -> (C, B, D) { (self.2, self.1, self.3) }
	fn bbr(self) -> (C, C, A) { (self.2, self.2, self.0) }
	fn bbg(self) -> (C, C, B) { (self.2, self.2, self.1) }
	fn bbb(self) -> (C, C, C) { (self.2, self.2, self.2) }
	fn bba(self) -> (C, C, D) { (self.2, self.2, self.3) }
	fn bar(self) -> (C, D, A) { (self.2, self.3, self.0) }
	fn bag(self) -> (C, D, B) { (self.2, self.3, self.1) }
	fn bab(self) -> (C, D, C) { (self.2, self.3, self.2) }
	fn baa(self) -> (C, D, D) { (self.2, self.3, self.3) }
	fn arr(self) -> (D, A, A) { (self.3, self.0, self.0) }
	fn arg(self) -> (D, A, B) { (self.3, self.0, self.1) }
	fn arb(self) -> (D, A, C) { (self.3, self.0, self.2) }
	fn ara(self) -> (D, A, D) { (self.3, self.0, self.3) }
	fn agr(self) -> (D, B, A) { (self.3, self.1, self.0) }
	fn agg(self) -> (D, B, B) { (self.3, self.1, self.1) }
	fn agb(self) -> (D, B, C) { (self.3, self.1, self.2) }
	fn aga(self) -> (D, B, D) { (self.3, self.1, self.3) }
	fn abr(self) -> (D, C, A) { (self.3, self.2, self.0) }
	fn abg(self) -> (D, C, B) { (self.3, self.2, self.1) }
	fn abb(self) -> (D, C, C) { (self.3, self.2, self.2) }
	fn aba(self) -> (D, C, D) { (self.3, self.2, self.3) }
	fn aar(self) -> (D, D, A) { (self.3, self.3, self.0) }
	fn aag(self) -> (D, D, B) { (self.3, self.3, self.1) }
	fn aab(self) -> (D, D, C) { (self.3, self.3, self.2) }
	fn aaa(self) -> (D, D, D) { (self.3, self.3, self.3) }

	fn rrrr(self) -> (A, A, A, A) { (self.0, self.0, self.0, self.0) }
	fn rrrg(self) -> (A, A, A, B) { (self.0, self.0, self.0, self.1) }
	fn rrrb(self) -> (A, A, A, C) { (self.0, self.0, self.0, self.2) }
	fn rrra(self) -> (A, A, A, D) { (self.0, self.0, self.0, self.3) }
	fn rrgr(self) -> (A, A, B, A) { (self.0, self.0, self.1, self.0) }
	fn rrgg(self) -> (A, A, B, B) { (self.0, self.0, self.1, self.1) }
	fn rrgb(self) -> (A, A, B, C) { (self.0, self.0, self.1, self.2) }
	fn rrga(self) -> (A, A, B, D) { (self.0, self.0, self.1, self.3) }
	fn rrbr(self) -> (A, A, C, A) { (self.0, self.0, self.2, self.0) }
	fn rrbg(self) -> (A, A, C, B) { (self.0, self.0, self.2, self.1) }
	fn rrbb(self) -> (A, A, C, C) { (self.0, self.0, self.2, self.2) }
	fn rrba(self) -> (A, A, C, D) { (self.0, self.0, self.2, self.3) }
	fn rrar(self) -> (A, A, D, A) { (self.0, self.0, self.3, self.0) }
	fn rrag(self) -> (A, A, D, B) { (self.0, self.0, self.3, self.1) }
	fn rrab(self) -> (A, A, D, C) { (self.0, self.0, self.3, self.2) }
	fn rraa(self) -> (A, A, D, D) { (self.0, self.0, self.3, self.3) }
	fn rgrr(self) -> (A, B, A, A) { (self.0, self.1, self.0, self.0) }
	fn rgrg(self) -> (A, B, A, B) { (self.0, self.1, self.0, self.1) }
	fn rgrb(self) -> (A, B, A, C) { (self.0, self.1, self.0, self.2) }
	fn rgra(self) -> (A, B, A, D) { (self.0, self.1, self.0, self.3) }
	fn rggr(self) -> (A, B, B, A) { (self.0, self.1, self.1, self.0) }
	fn rggg(self) -> (A, B, B, B) { (self.0, self.1, self.1, self.1) }
	fn rggb(self) -> (A, B, B, C) { (self.0, self.1, self.1, self.2) }
	fn rgga(self) -> (A, B, B, D) { (self.0, self.1, self.1, self.3) }
	fn rgbr(self) -> (A, B, C, A) { (self.0, self.1, self.2, self.0) }
	fn rgbg(self) -> (A, B, C, B) { (self.0, self.1, self.2, self.1) }
	fn rgbb(self) -> (A, B, C, C) { (self.0, self.1, self.2, self.2) }
	fn rgba(self) -> (A, B, C, D) { (self.0, self.1, self.2, self.3) }
	fn rgar(self) -> (A, B, D, A) { (self.0, self.1, self.3, self.0) }
	fn rgag(self) -> (A, B, D, B) { (self.0, self.1, self.3, self.1) }
	fn rgab(self) -> (A, B, D, C) { (self.0, self.1, self.3, self.2) }
	fn rgaa(self) -> (A, B, D, D) { (self.0, self.1, self.3, self.3) }
	fn rbrr(self) -> (A, C, A, A) { (self.0, self.2, self.0, self.0) }
	fn rbrg(self) -> (A, C, A, B) { (self.0, self.2, self.0, self.1) }
	fn rbrb(self) -> (A, C, A, C) { (self.0, self.2, self.0, self.2) }
	fn rbra(self) -> (A, C, A, D) { (self.0, self.2, self.0, self.3) }
	fn rbgr(self) -> (A, C, B, A) { (self.0, self.2, self.1, self.0) }
	fn rbgg(self) -> (A, C, B, B) { (self.0, self.2, self.1, self.1) }
	fn rbgb(self) -> (A, C, B, C) { (self.0, self.2, self.1, self.2) }
	fn rbga(self) -> (A, C, B, D) { (self.0, self.2, self.1, self.3) }
	fn rbbr(self) -> (A, C, C, A) { (self.0, self.2, self.2, self.0) }
	fn rbbg(self) -> (A, C, C, B) { (self.0, self.2, self.2, self.1) }
	fn rbbb(self) -> (A, C, C, C) { (self.0, self.2, self.2, self.2) }
	fn rbba(self) -> (A, C, C, D) { (self.0, self.2, self.2, self.3) }
	fn rbar(self) -> (A, C, D, A) { (self.0, self.2, self.3, self.0) }
	fn rbag(self) -> (A, C, D, B) { (self.0, self.2, self.3, self.1) }
	fn rbab(self) -> (A, C, D, C) { (self.0, self.2, self.3, self.2) }
	fn rbaa(self) -> (A, C, D, D) { (self.0, self.2, self.3, self.3) }
	fn rarr(self) -> (A, D, A, A) { (self.0, self.3, self.0, self.0) }
	fn rarg(self) -> (A, D, A, B) { (self.0, self.3, self.0, self.1) }
	fn rarb(self) -> (A, D, A, C) { (self.0, self.3, self.0, self.2) }
	fn rara(self) -> (A, D, A, D) { (self.0, self.3, self.0, self.3) }
	fn ragr(self) -> (A, D, B, A) { (self.0, self.3, self.1, self.0) }
	fn ragg(self) -> (A, D, B, B) { (self.0, self.3, self.1, self.1) }
	fn ragb(self) -> (A, D, B, C) { (self.0, self.3, self.1, self.2) }
	fn raga(self) -> (A, D, B, D) { (self.0, self.3, self.1, self.3) }
	fn rabr(self) -> (A, D, C, A) { (self.0, self.3, self.2, self.0) }
	fn rabg(self) -> (A, D, C, B) { (self.0, self.3, self.2, self.1) }
	fn rabb(self) -> (A, D, C, C) { (self.0, self.3, self.2, self.2) }
	fn raba(self) -> (A, D, C, D) { (self.0, self.3, self.2, self.3) }
	fn raar(self) -> (A, D, D, A) { (self.0, self.3, self.3, self.0) }
	fn raag(self) -> (A, D, D, B) { (self.0, self.3, self.3, self.1) }
	fn raab(self) -> (A, D, D, C) { (self.0, self.3, self.3, self.2) }
	fn raaa(self) -> (A, D, D, D) { (self.0, self.3, self.3, self.3) }

	fn grrr(self) -> (B, A, A, A) { (self.1, self.0, self.0, self.0) }
	fn grrg(self) -> (B, A, A, B) { (self.1, self.0, self.0, self.1) }
	fn grrb(self) -> (B, A, A, C) { (self.1, self.0, self.0, self.2) }
	fn grra(self) -> (B, A, A, D) { (self.1, self.0, self.0, self.3) }
	fn grgr(self) -> (B, A, B, A) { (self.1, self.0, self.1, self.0) }
	fn grgg(self) -> (B, A, B, B) { (self.1, self.0, self.1, self.1) }
	fn grgb(self) -> (B, A, B, C) { (self.1, self.0, self.1, self.2) }
	fn grga(self) -> (B, A, B, D) { (self.1, self.0, self.1, self.3) }
	fn grbr(self) -> (B, A, C, A) { (self.1, self.0, self.2, self.0) }
	fn grbg(self) -> (B, A, C, B) { (self.1, self.0, self.2, self.1) }
	fn grbb(self) -> (B, A, C, C) { (self.1, self.0, self.2, self.2) }
	fn grba(self) -> (B, A, C, D) { (self.1, self.0, self.2, self.3) }
	fn grar(self) -> (B, A, D, A) { (self.1, self.0, self.3, self.0) }
	fn grag(self) -> (B, A, D, B) { (self.1, self.0, self.3, self.1) }
	fn grab(self) -> (B, A, D, C) { (self.1, self.0, self.3, self.2) }
	fn graa(self) -> (B, A, D, D) { (self.1, self.0, self.3, self.3) }
	fn ggrr(self) -> (B, B, A, A) { (self.1, self.1, self.0, self.0) }
	fn ggrg(self) -> (B, B, A, B) { (self.1, self.1, self.0, self.1) }
	fn ggrb(self) -> (B, B, A, C) { (self.1, self.1, self.0, self.2) }
	fn ggra(self) -> (B, B, A, D) { (self.1, self.1, self.0, self.3) }
	fn gggr(self) -> (B, B, B, A) { (self.1, self.1, self.1, self.0) }
	fn gggg(self) -> (B, B, B, B) { (self.1, self.1, self.1, self.1) }
	fn gggb(self) -> (B, B, B, C) { (self.1, self.1, self.1, self.2) }
	fn ggga(self) -> (B, B, B, D) { (self.1, self.1, self.1, self.3) }
	fn ggbr(self) -> (B, B, C, A) { (self.1, self.1, self.2, self.0) }
	fn ggbg(self) -> (B, B, C, B) { (self.1, self.1, self.2, self.1) }
	fn ggbb(self) -> (B, B, C, C) { (self.1, self.1, self.2, self.2) }
	fn ggba(self) -> (B, B, C, D) { (self.1, self.1, self.2, self.3) }
	fn ggar(self) -> (B, B, D, A) { (self.1, self.1, self.3, self.0) }
	fn ggag(self) -> (B, B, D, B) { (self.1, self.1, self.3, self.1) }
	fn ggab(self) -> (B, B, D, C) { (self.1, self.1, self.3, self.2) }
	fn ggaa(self) -> (B, B, D, D) { (self.1, self.1, self.3, self.3) }
	fn gbrr(self) -> (B, C, A, A) { (self.1, self.2, self.0, self.0) }
	fn gbrg(self) -> (B, C, A, B) { (self.1, self.2, self.0, self.1) }
	fn gbrb(self) -> (B, C, A, C) { (self.1, self.2, self.0, self.2) }
	fn gbra(self) -> (B, C, A, D) { (self.1, self.2, self.0, self.3) }
	fn gbgr(self) -> (B, C, B, A) { (self.1, self.2, self.1, self.0) }
	fn gbgg(self) -> (B, C, B, B) { (self.1, self.2, self.1, self.1) }
	fn gbgb(self) -> (B, C, B, C) { (self.1, self.2, self.1, self.2) }
	fn gbga(self) -> (B, C, B, D) { (self.1, self.2, self.1, self.3) }
	fn gbbr(self) -> (B, C, C, A) { (self.1, self.2, self.2, self.0) }
	fn gbbg(self) -> (B, C, C, B) { (self.1, self.2, self.2, self.1) }
	fn gbbb(self) -> (B, C, C, C) { (self.1, self.2, self.2, self.2) }
	fn gbba(self) -> (B, C, C, D) { (self.1, self.2, self.2, self.3) }
	fn gbar(self) -> (B, C, D, A) { (self.1, self.2, self.3, self.0) }
	fn gbag(self) -> (B, C, D, B) { (self.1, self.2, self.3, self.1) }
	fn gbab(self) -> (B, C, D, C) { (self.1, self.2, self.3, self.2) }
	fn gbaa(self) -> (B, C, D, D) { (self.1, self.2, self.3, self.3) }
	fn garr(self) -> (B, D, A, A) { (self.1, self.3, self.0, self.0) }
	fn garg(self) -> (B, D, A, B) { (self.1, self.3, self.0, self.1) }
	fn garb(self) -> (B, D, A, C) { (self.1, self.3, self.0, self.2) }
	fn gara(self) -> (B, D, A, D) { (self.1, self.3, self.0, self.3) }
	fn gagr(self) -> (B, D, B, A) { (self.1, self.3, self.1, self.0) }
	fn gagg(self) -> (B, D, B, B) { (self.1, self.3, self.1, self.1) }
	fn gagb(self) -> (B, D, B, C) { (self.1, self.3, self.1, self.2) }
	fn gaga(self) -> (B, D, B, D) { (self.1, self.3, self.1, self.3) }
	fn gabr(self) -> (B, D, C, A) { (self.1, self.3, self.2, self.0) }
	fn gabg(self) -> (B, D, C, B) { (self.1, self.3, self.2, self.1) }
	fn gabb(self) -> (B, D, C, C) { (self.1, self.3, self.2, self.2) }
	fn gaba(self) -> (B, D, C, D) { (self.1, self.3, self.2, self.3) }
	fn gaar(self) -> (B, D, D, A) { (self.1, self.3, self.3, self.0) }
	fn gaag(self) -> (B, D, D, B) { (self.1, self.3, self.3, self.1) }
	fn gaab(self) -> (B, D, D, C) { (self.1, self.3, self.3, self.2) }
	fn gaaa(self) -> (B, D, D, D) { (self.1, self.3, self.3, self.3) }

	fn brrr(self) -> (C, A, A, A) { (self.2, self.0, self.0, self.0) }
	fn brrg(self) -> (C, A, A, B) { (self.2, self.0, self.0, self.1) }
	fn brrb(self) -> (C, A, A, C) { (self.2, self.0, self.0, self.2) }
	fn brra(self) -> (C, A, A, D) { (self.2, self.0, self.0, self.3) }
	fn brgr(self) -> (C, A, B, A) { (self.2, self.0, self.1, self.0) }
	fn brgg(self) -> (C, A, B, B) { (self.2, self.0, self.1, self.1) }
	fn brgb(self) -> (C, A, B, C) { (self.2, self.0, self.1, self.2) }
	fn brga(self) -> (C, A, B, D) { (self.2, self.0, self.1, self.3) }
	fn brbr(self) -> (C, A, C, A) { (self.2, self.0, self.2, self.0) }
	fn brbg(self) -> (C, A, C, B) { (self.2, self.0, self.2, self.1) }
	fn brbb(self) -> (C, A, C, C) { (self.2, self.0, self.2, self.2) }
	fn brba(self) -> (C, A, C, D) { (self.2, self.0, self.2, self.3) }
	fn brar(self) -> (C, A, D, A) { (self.2, self.0, self.3, self.0) }
	fn brag(self) -> (C, A, D, B) { (self.2, self.0, self.3, self.1) }
	fn brab(self) -> (C, A, D, C) { (self.2, self.0, self.3, self.2) }
	fn braa(self) -> (C, A, D, D) { (self.2, self.0, self.3, self.3) }
	fn bgrr(self) -> (C, B, A, A) { (self.2, self.1, self.0, self.0) }
	fn bgrg(self) -> (C, B, A, B) { (self.2, self.1, self.0, self.1) }
	fn bgrb(self) -> (C, B, A, C) { (self.2, self.1, self.0, self.2) }
	fn bgra(self) -> (C, B, A, D) { (self.2, self.1, self.0, self.3) }
	fn bggr(self) -> (C, B, B, A) { (self.2, self.1, self.1, self.0) }
	fn bggg(self) -> (C, B, B, B) { (self.2, self.1, self.1, self.1) }
	fn bggb(self) -> (C, B, B, C) { (self.2, self.1, self.1, self.2) }
	fn bgga(self) -> (C, B, B, D) { (self.2, self.1, self.1, self.3) }
	fn bgbr(self) -> (C, B, C, A) { (self.2, self.1, self.2, self.0) }
	fn bgbg(self) -> (C, B, C, B) { (self.2, self.1, self.2, self.1) }
	fn bgbb(self) -> (C, B, C, C) { (self.2, self.1, self.2, self.2) }
	fn bgba(self) -> (C, B, C, D) { (self.2, self.1, self.2, self.3) }
	fn bgar(self) -> (C, B, D, A) { (self.2, self.1, self.3, self.0) }
	fn bgag(self) -> (C, B, D, B) { (self.2, self.1, self.3, self.1) }
	fn bgab(self) -> (C, B, D, C) { (self.2, self.1, self.3, self.2) }
	fn bgaa(self) -> (C, B, D, D) { (self.2, self.1, self.3, self.3) }
	fn bbrr(self) -> (C, C, A, A) { (self.2, self.2, self.0, self.0) }
	fn bbrg(self) -> (C, C, A, B) { (self.2, self.2, self.0, self.1) }
	fn bbrb(self) -> (C, C, A, C) { (self.2, self.2, self.0, self.2) }
	fn bbra(self) -> (C, C, A, D) { (self.2, self.2, self.0, self.3) }
	fn bbgr(self) -> (C, C, B, A) { (self.2, self.2, self.1, self.0) }
	fn bbgg(self) -> (C, C, B, B) { (self.2, self.2, self.1, self.1) }
	fn bbgb(self) -> (C, C, B, C) { (self.2, self.2, self.1, self.2) }
	fn bbga(self) -> (C, C, B, D) { (self.2, self.2, self.1, self.3) }
	fn bbbr(self) -> (C, C, C, A) { (self.2, self.2, self.2, self.0) }
	fn bbbg(self) -> (C, C, C, B) { (self.2, self.2, self.2, self.1) }
	fn bbbb(self) -> (C, C, C, C) { (self.2, self.2, self.2, self.2) }
	fn bbba(self) -> (C, C, C, D) { (self.2, self.2, self.2, self.3) }
	fn bbar(self) -> (C, C, D, A) { (self.2, self.2, self.3, self.0) }
	fn bbag(self) -> (C, C, D, B) { (self.2, self.2, self.3, self.1) }
	fn bbab(self) -> (C, C, D, C) { (self.2, self.2, self.3, self.2) }
	fn bbaa(self) -> (C, C, D, D) { (self.2, self.2, self.3, self.3) }
	fn barr(self) -> (C, D, A, A) { (self.2, self.3, self.0, self.0) }
	fn barg(self) -> (C, D, A, B) { (self.2, self.3, self.0, self.1) }
	fn barb(self) -> (C, D, A, C) { (self.2, self.3, self.0, self.2) }
	fn bara(self) -> (C, D, A, D) { (self.2, self.3, self.0, self.3) }
	fn bagr(self) -> (C, D, B, A) { (self.2, self.3, self.1, self.0) }
	fn bagg(self) -> (C, D, B, B) { (self.2, self.3, self.1, self.1) }
	fn bagb(self) -> (C, D, B, C) { (self.2, self.3, self.1, self.2) }
	fn baga(self) -> (C, D, B, D) { (self.2, self.3, self.1, self.3) }
	fn babr(self) -> (C, D, C, A) { (self.2, self.3, self.2, self.0) }
	fn babg(self) -> (C, D, C, B) { (self.2, self.3, self.2, self.1) }
	fn babb(self) -> (C, D, C, C) { (self.2, self.3, self.2, self.2) }
	fn baba(self) -> (C, D, C, D) { (self.2, self.3, self.2, self.3) }
	fn baar(self) -> (C, D, D, A) { (self.2, self.3, self.3, self.0) }
	fn baag(self) -> (C, D, D, B) { (self.2, self.3, self.3, self.1) }
	fn baab(self) -> (C, D, D, C) { (self.2, self.3, self.3, self.2) }
	fn baaa(self) -> (C, D, D, D) { (self.2, self.3, self.3, self.3) }

	fn arrr(self) -> (D, A, A, A) { (self.3, self.0, self.0, self.0) }
	fn arrg(self) -> (D, A, A, B) { (self.3, self.0, self.0, self.1) }
	fn arrb(self) -> (D, A, A, C) { (self.3, self.0, self.0, self.2) }
	fn arra(self) -> (D, A, A, D) { (self.3, self.0, self.0, self.3) }
	fn argr(self) -> (D, A, B, A) { (self.3, self.0, self.1, self.0) }
	fn argg(self) -> (D, A, B, B) { (self.3, self.0, self.1, self.1) }
	fn argb(self) -> (D, A, B, C) { (self.3, self.0, self.1, self.2) }
	fn arga(self) -> (D, A, B, D) { (self.3, self.0, self.1, self.3) }
	fn arbr(self) -> (D, A, C, A) { (self.3, self.0, self.2, self.0) }
	fn arbg(self) -> (D, A, C, B) { (self.3, self.0, self.2, self.1) }
	fn arbb(self) -> (D, A, C, C) { (self.3, self.0, self.2, self.2) }
	fn arba(self) -> (D, A, C, D) { (self.3, self.0, self.2, self.3) }
	fn arar(self) -> (D, A, D, A) { (self.3, self.0, self.3, self.0) }
	fn arag(self) -> (D, A, D, B) { (self.3, self.0, self.3, self.1) }
	fn arab(self) -> (D, A, D, C) { (self.3, self.0, self.3, self.2) }
	fn araa(self) -> (D, A, D, D) { (self.3, self.0, self.3, self.3) }
	fn agrr(self) -> (D, B, A, A) { (self.3, self.1, self.0, self.0) }
	fn agrg(self) -> (D, B, A, B) { (self.3, self.1, self.0, self.1) }
	fn agrb(self) -> (D, B, A, C) { (self.3, self.1, self.0, self.2) }
	fn agra(self) -> (D, B, A, D) { (self.3, self.1, self.0, self.3) }
	fn aggr(self) -> (D, B, B, A) { (self.3, self.1, self.1, self.0) }
	fn aggg(self) -> (D, B, B, B) { (self.3, self.1, self.1, self.1) }
	fn aggb(self) -> (D, B, B, C) { (self.3, self.1, self.1, self.2) }
	fn agga(self) -> (D, B, B, D) { (self.3, self.1, self.1, self.3) }
	fn agbr(self) -> (D, B, C, A) { (self.3, self.1, self.2, self.0) }
	fn agbg(self) -> (D, B, C, B) { (self.3, self.1, self.2, self.1) }
	fn agbb(self) -> (D, B, C, C) { (self.3, self.1, self.2, self.2) }
	fn agba(self) -> (D, B, C, D) { (self.3, self.1, self.2, self.3) }
	fn agar(self) -> (D, B, D, A) { (self.3, self.1, self.3, self.0) }
	fn agag(self) -> (D, B, D, B) { (self.3, self.1, self.3, self.1) }
	fn agab(self) -> (D, B, D, C) { (self.3, self.1, self.3, self.2) }
	fn agaa(self) -> (D, B, D, D) { (self.3, self.1, self.3, self.3) }
	fn abrr(self) -> (D, C, A, A) { (self.3, self.2, self.0, self.0) }
	fn abrg(self) -> (D, C, A, B) { (self.3, self.2, self.0, self.1) }
	fn abrb(self) -> (D, C, A, C) { (self.3, self.2, self.0, self.2) }
	fn abra(self) -> (D, C, A, D) { (self.3, self.2, self.0, self.3) }
	fn abgr(self) -> (D, C, B, A) { (self.3, self.2, self.1, self.0) }
	fn abgg(self) -> (D, C, B, B) { (self.3, self.2, self.1, self.1) }
	fn abgb(self) -> (D, C, B, C) { (self.3, self.2, self.1, self.2) }
	fn abga(self) -> (D, C, B, D) { (self.3, self.2, self.1, self.3) }
	fn abbr(self) -> (D, C, C, A) { (self.3, self.2, self.2, self.0) }
	fn abbg(self) -> (D, C, C, B) { (self.3, self.2, self.2, self.1) }
	fn abbb(self) -> (D, C, C, C) { (self.3, self.2, self.2, self.2) }
	fn abba(self) -> (D, C, C, D) { (self.3, self.2, self.2, self.3) }
	fn abar(self) -> (D, C, D, A) { (self.3, self.2, self.3, self.0) }
	fn abag(self) -> (D, C, D, B) { (self.3, self.2, self.3, self.1) }
	fn abab(self) -> (D, C, D, C) { (self.3, self.2, self.3, self.2) }
	fn abaa(self) -> (D, C, D, D) { (self.3, self.2, self.3, self.3) }
	fn aarr(self) -> (D, D, A, A) { (self.3, self.3, self.0, self.0) }
	fn aarg(self) -> (D, D, A, B) { (self.3, self.3, self.0, self.1) }
	fn aarb(self) -> (D, D, A, C) { (self.3, self.3, self.0, self.2) }
	fn aara(self) -> (D, D, A, D) { (self.3, self.3, self.0, self.3) }
	fn aagr(self) -> (D, D, B, A) { (self.3, self.3, self.1, self.0) }
	fn aagg(self) -> (D, D, B, B) { (self.3, self.3, self.1, self.1) }
	fn aagb(self) -> (D, D, B, C) { (self.3, self.3, self.1, self.2) }
	fn aaga(self) -> (D, D, B, D) { (self.3, self.3, self.1, self.3) }
	fn aabr(self) -> (D, D, C, A) { (self.3, self.3, self.2, self.0) }
	fn aabg(self) -> (D, D, C, B) { (self.3, self.3, self.2, self.1) }
	fn aabb(self) -> (D, D, C, C) { (self.3, self.3, self.2, self.2) }
	fn aaba(self) -> (D, D, C, D) { (self.3, self.3, self.2, self.3) }
	fn aaar(self) -> (D, D, D, A) { (self.3, self.3, self.3, self.0) }
	fn aaag(self) -> (D, D, D, B) { (self.3, self.3, self.3, self.1) }
	fn aaab(self) -> (D, D, D, C) { (self.3, self.3, self.3, self.2) }
	fn aaaa(self) -> (D, D, D, D) { (self.3, self.3, self.3, self.3) }
}
}
