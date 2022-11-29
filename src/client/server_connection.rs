trait ServerConnection<In, Out> {
    fn send(&mut self, data: Out);
    fn rcv_next(&mut self) -> Option<In>;
}
