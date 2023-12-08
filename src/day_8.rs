pub mod part_1 {
    use std::collections::HashMap;

    const ZZZ: u32 = u32::from_be_bytes(*b"\0ZZZ");

    pub fn solution(input: String) -> usize {
        let (instructions, path) = parse(&input);
        let mut steps = 0;
        let mut current = u32::from_be_bytes(*b"\0AAA");

        while current != ZZZ {
            let direction = instructions[steps % instructions.len()];

            if direction {
                current = path.get(&current).unwrap().1;
            } else {
                current = path.get(&current).unwrap().0;
            }

            steps += 1;
        }

        steps
    }

    fn parse(input: &str) -> (Vec<bool>, HashMap<u32, (u32, u32)>) {
        let (instructions, parts) = input.split_once("\n\n").unwrap();
        let mut branches = HashMap::with_capacity(parts.len() / 16);

        for part in parts.lines() {
            let (k, v) = part
                .split_once(" = (")
                .map(|(name, branch)| (name.as_bytes(), branch))
                .map(|(name, branch)| {
                    (
                        u32::from_be_bytes([0, name[0], name[1], name[2]]),
                        branch
                            .split_once(',')
                            .map(|a| (a.0, &a.1[1..a.1.len() - 1]))
                            .map(|(l, r)| (l.as_bytes(), r.as_bytes()))
                            .map(|(l, r)| (u32::from_be_bytes([0, l[0], l[1], l[2]]), u32::from_be_bytes([0 ,r[0], r[1], r[2]])))
                            .unwrap(),
                    )
                })
                .unwrap();

            branches.insert(k, v);
        }

        (instructions.as_bytes().iter().map(|b| *b == b'R').collect(), branches)
    }

    #[test]
    pub fn test() {
        crate::util::test_solution(8, solution, 13207);
    }
}

pub mod part_2 {
    use std::collections::HashMap;

    const A: u8 = u8::from_be_bytes(*b"A");
    const Z: u8 = u8::from_be_bytes(*b"Z");
    type Path<'a> = HashMap<u32, (u32, u32)>;

    pub fn solution(input: String) -> usize {
        let (instructions, path) = parse(&input);
        path
            .keys()
            .filter(|k| k.to_be_bytes()[3] == A)
            .map(|v| get_path_length(&instructions, &path, *v))
            .fold(1, |acc, b| crate::util::lcm(acc, b))
    }

    fn get_path_length(instructions: &Vec<bool>, path: &Path, start: u32) -> usize {
        let mut steps = 0;
        let mut current = start;

        while current.to_be_bytes()[3] != Z {
            let direction = instructions[steps % instructions.len()];

            if direction {
                current = path.get(&current).unwrap().1;
            } else {
                current = path.get(&current).unwrap().0;
            }

            steps += 1;
        }

        steps
    }

    fn parse(input: &str) -> (Vec<bool>, Path) {
        let (instructions, parts) = input.split_once("\n\n").unwrap();
        let mut branches = HashMap::with_capacity(parts.len() / 16);

        for part in parts.lines() {
            let (k, v) = part
                .split_once(" = (")
                .map(|(name, branch)| (name.as_bytes(), branch))
                .map(|(name, branch)| {
                    (
                        u32::from_be_bytes([0, name[0], name[1], name[2]]),
                        branch
                            .split_once(',')
                            .map(|a| (a.0, &a.1[1..a.1.len() - 1]))
                            .map(|(l, r)| (l.as_bytes(), r.as_bytes()))
                            .map(|(l, r)| (u32::from_be_bytes([0, l[0], l[1], l[2]]), u32::from_be_bytes([0 ,r[0], r[1], r[2]])))
                            .unwrap(),
                    )
                })
                .unwrap();

            branches.insert(k, v);
        }

        (instructions.as_bytes().iter().map(|b| *b == b'R').collect(), branches)
    }

    #[test]
    pub fn test() {
        crate::util::test_solution(8, solution, 12324145107121);
    }
}
