#![feature(bool_to_option)]

type Link = (u8, u8);
type Chain = Vec<Link>;

trait ChainOps {
    fn first_last(&self) -> Link;
    fn chain_reverse(&mut self);
}

impl ChainOps for Link {
    fn first_last(&self) -> Link {
        *self
    }

    fn chain_reverse(&mut self) {
        let (a, b) = self;
        std::mem::swap(&mut (*a), &mut (*b));
    }
}

impl ChainOps for Chain {
    fn first_last(&self) -> Link {
        let &(f, _) = self.first().unwrap();
        let &(_, l) = self.last().unwrap();
        (f, l)
    }

    fn chain_reverse(&mut self) {
        self.reverse();
        self.iter_mut().for_each(<Link>::chain_reverse);
    }
}

fn linkage<L: ChainOps, R: ChainOps>(l: &L, r: &R) -> bool {
    let (a, b) = l.first_last();
    let (c, d) = r.first_last();
    a == c || a == d || b == c || b == d
}

pub fn chain(input: &[Link]) -> Option<Chain> {
    if input.is_empty() {
        return Some(vec![]);
    }

    let mut links: Chain = input
        .iter()
        .map(|&(a, b)| if a <= b { (a, b) } else { (b, a) })
        .collect();

    links.sort();

    let mut chain: Chain = vec![links.pop().unwrap()];

    while !links.is_empty() {
        if let Some(i) = links
            .iter()
            .enumerate()
            .filter_map(|(i, link)| linkage(&chain, link).then_some(i))
            .next()
        {
            let mut link = links.remove(i);
            let (a, b) = chain.first_last();
            let (c, d) = link.first_last();

            if a == c {
                chain.chain_reverse();
            } else if b == d {
                link.chain_reverse();
            } else if a == d {
                chain.chain_reverse();
                link.chain_reverse();
            }

            chain.push(link);
        } else {
            return None;
        }
    }

    let (a, b) = chain.first_last();
    (a == b).then_some(chain)
}
