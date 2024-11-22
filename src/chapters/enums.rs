
#[derive(Debug)]
struct V4Addr {
    
}

#[derive(Debug)]
struct V6Addr{

}

#[derive(Debug)]
pub enum IpAddr{
    V4(V4Addr),
    V6(V6Addr),
}

