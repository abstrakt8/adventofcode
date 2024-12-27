import re
from graphviz import Digraph

# Function to parse the input text
def parse_input(file_path):
    operations = []
    with open(file_path, "r") as file:
        lines = file.readlines()
        for line in lines:
            match = re.match(r"(.+?)\s+(\w+)\s+(.+?)\s+->\s+(.+)", line.strip())
            if match:
                source1, operation, source2, target = match.groups()
                operations.append((source1.strip(), operation.strip(), source2.strip(), target.strip()))
    return operations

# Function to calculate yellow shades based on ranges
def calculate_yellow_shade(number):
    # Map ranges to specific yellow shades
    if 0 <= number < 5:
        return "#ffffe0"  # Very light yellow
    elif 5 <= number < 10:
        return "#fffacd"  # Light yellow
    elif 10 <= number < 15:
        return "#ffebcd"  # Slightly darker yellow
    elif 15 <= number < 20:
        return "#ffe4b5"  # Medium light yellow
    elif 20 <= number < 25:
        return "#ffd700"  # Golden yellow
    elif 25 <= number < 30:
        return "#ffc107"  # Bright yellow
    elif 30 <= number < 35:
        return "#ffb000"  # Deeper yellow
    elif 35 <= number < 40:
        return "#ffa500"  # Orange-yellow
    elif 40 <= number <= 45:
        return "#ff8c00"  # Darker yellow-orange
    else:
        return "#ffffff"  # Default white for out-of-range values

# Function to generate the graph
def generate_graph(operations, output_filename="logic_graph"):
    dot = Digraph("LogicGraph", format="png")
    dot.attr(rankdir="LR")
    
    gate_counter = 1  # Counter to create unique gate node names
    or_nodes = []  # Keep track of OR nodes to align them later

    for source1, operation, source2, target in operations:
        # Create a unique gate node name
        gate_name = f"{operation}_{gate_counter}"
        gate_counter += 1

        # Determine the color based on the operation type
        if operation == "AND":
            gate_color = "red"
        elif operation == "OR":
            gate_color = "green"
            or_nodes.append(gate_name)  # Add to OR node list
        elif operation == "XOR":
            gate_color = "blue"
        else:
            gate_color = "black"  # Default color for unknown operations

        # Add the gate node with the operation name and color
        dot.node(gate_name, label=operation, shape="ellipse", style="filled", fillcolor=gate_color)

        # Helper function to apply yellow shade color to nodes matching x{number}, y{number}, z{number}
        def add_shaded_node(node_name):
            match = re.match(r"[xyz](\d+)", node_name)
            if match:
                number = int(match.group(1))
                shade_color = calculate_yellow_shade(number)
                dot.node(node_name, label=node_name, style="filled", fillcolor=shade_color)
            else:
                dot.node(node_name, label=node_name)  # Regular node for non-matching names

        # Add the source nodes with yellow shades if applicable
        add_shaded_node(source1)
        add_shaded_node(source2)
        add_shaded_node(target)

        # Connect the input wires to the gate node
        dot.edge(source1, gate_name)
        dot.edge(source2, gate_name)

        # Connect the gate node to the target wire
        dot.edge(gate_name, target)

    # Align all OR nodes on the same horizontal line
    if or_nodes:
        with dot.subgraph() as s:
            s.attr(rank="same")
            for node in or_nodes:
                s.node(node)

    dot.render(output_filename, view=True)

def swap_targets(operations, target1, target2):
    swapped = False
    for i, (source1, operation, source2, target) in enumerate(operations):
        if target == target1:
            operations[i] = (source1, operation, source2, target2)
            swapped = True
        elif target == target2:
            operations[i] = (source1, operation, source2, target1)
            swapped = True
    return swapped

def interactive_mode(operations):
    swaps = []
    while True:
        command = input("Enter command (swap/undo/stop): ").strip().lower()

        if command == "swap":
            target1 = input("Enter first target to swap: ").strip()
            target2 = input("Enter second target to swap: ").strip()

            if swap_targets(operations, target1, target2):
                swaps.append((target1, target2))
                print(f"Swapped {target1} and {target2}")
                generate_graph(operations, f"swapped_{len(swaps)}")
            else:
                print("Swap failed. One or both targets not found.")

        elif command == "undo":
            if swaps:
                last_swap = swaps.pop()
                swap_targets(operations, last_swap[0], last_swap[1])
                print(f"Undone swap of {last_swap[0]} and {last_swap[1]}")
                generate_graph(operations, f"undone_{len(swaps)}")
            else:
                print("No swaps to undo.")

        elif command == "stop":
            print("Exiting interactive mode.")
            print("Swaps performed:", swaps)
            break

        else:
            print("Invalid command. Please use 'swap', 'undo', or 'stop'.")

# Main function
if __name__ == "__main__":
    input_file = "input.txt"  # Ensure the input.txt file exists in the same directory
    operations = parse_input(input_file)

    generate_graph(operations, "initial_graph")

    print("Initial graph generated. Entering interactive mode.")
    interactive_mode(operations)

# Swaps done manually
# z13, npf
# z19, cph
# z33, hgj
# nnt, gws
