pub mod part_1 {
    pub fn solution(input: String) -> u32 {
        let (mut seeds, maps) = parse(&input);

        for map in maps.iter() {
            let mut next = Vec::new();

            for seed in seeds.iter() {
                let mut result = *seed;

                for [dest, source, len] in map {
                    if seed > source && seed - source < *len {
                        result = seed - source + dest;
                    }
                }

                next.push(result);
            }

            seeds = next;
        }

        *seeds.iter().min().unwrap()
    }

    pub fn parse(input: &str) -> (Vec<u32>, Vec<Vec<[u32; 3]>>) {
        let parts = input.split("\n\n").collect::<Vec<_>>();
        let mut seeds = Vec::with_capacity(4);

        for seed in parts[0][7..].split(' ') {
            seeds.push(seed.parse::<u32>().unwrap());
        }

        let mut maps = Vec::new();

        for part in parts.iter().skip(1) {
            let (_, triples) = part.split_once('\n').unwrap();
            let mut map = Vec::new();

            for triple in triples.split('\n') {
                let mut triple = triple.split(' ');

                map.push([
                    triple.next().unwrap().parse::<u32>().unwrap(),
                    triple.next().unwrap().parse::<u32>().unwrap(),
                    triple.next().unwrap().parse::<u32>().unwrap(),
                ]);
            }

            maps.push(map);
        }

        (seeds, maps)
    }

    #[test]
    pub fn test() {
        crate::util::test_solution(5, solution, 31599214);
    }
}

pub mod part_2 {
    pub fn solution(input: String) -> u32 {
        let (seeds, maps) = parse(&input);
        let mut locations = Vec::new();

        for (start, len) in seeds.into_iter() {
            locations.push(locations_for_seeds((start..(start + len)).collect(), &maps));
        }

        *locations.iter().min().unwrap()
    }

    pub fn locations_for_seeds(mut seeds: Vec<u32>, maps: &[Vec<[u32; 3]>]) -> u32 {
        for map in maps.iter() {
            let mut next = Vec::new();

            for seed in seeds.iter() {
                let mut result = *seed;

                for [dest, source, len] in map {
                    if seed >= source && seed - source < *len {
                        result = seed - source + dest;
                    }
                }

                next.push(result);
            }

            seeds = next;
        }

        *seeds.iter().min().unwrap()
    }

    type ParseResult = (Vec<(u32, u32)>, Vec<Vec<[u32; 3]>>);

    pub fn parse(input: &str) -> ParseResult {
        let parts = input.split("\n\n").collect::<Vec<_>>();
        let mut seeds = Vec::new();
        let mut iter = parts[0][7..].split(' ');

        while let (Some(start), Some(len)) = (iter.next(), iter.next()) {
            let (start, len) = (start.parse::<u32>().unwrap(), len.parse::<u32>().unwrap());
            seeds.push((start, len));
        }

        let mut maps = Vec::new();

        for part in parts.iter().skip(1) {
            let (_, triples) = part.split_once('\n').unwrap();
            let mut map = Vec::new();

            for triple in triples.split('\n') {
                let mut triple = triple.split(' ');

                map.push([
                    triple.next().unwrap().parse::<u32>().unwrap(),
                    triple.next().unwrap().parse::<u32>().unwrap(),
                    triple.next().unwrap().parse::<u32>().unwrap(),
                ]);
            }

            maps.push(map);
        }

        (seeds, maps)
    }
}
