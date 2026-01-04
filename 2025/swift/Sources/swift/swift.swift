// The Swift Programming Language
// https://docs.swift.org/swift-book
import Foundation

@main
struct swift {

    static func main() {
        do {
            let content = try String(contentsOfFile: "../examples/p5.in", encoding: .utf8)
            // let lines = content.split(separator: "\n");
            // let chunks = lines.split(whereSeparator: { $0.isEmpty } )
            //
            let lines = content.components(separatedBy: .newlines)
            let chunks = lines.split(whereSeparator: { $0.isEmpty })
            var freshness = chunks[0].map({
                let t = $0.split(separator: "-")
                let a = Int64(t[0])!
                let b = Int64(t[1])!
                return (a, b)
            })
            let ingredients = chunks[1].map({ Int($0)! })

            var ans1 = 0
            for val in ingredients {
                let isFresh: Bool = {
                    for (a, b) in freshness {
                        if a <= val && val <= b {
                            return true
                        }
                    }
                    return false
                }()
                if isFresh {
                    ans1 += 1
                }
            }

            print(ans1)

            freshness.sort(by: { $0.0 < $1.0 })
            var end: Int64 = 0
            var ans2 = Int64(0)
            for (a, b) in freshness {
                if end < a {
                    end = b
                    ans2 += b - a + 1
                } else if end < b {
                    ans2 += b - end
                    end = b
                }
            }
            print(ans2)
        } catch {
            print("Hello, world!")
        }
    }
}
