import time
import os

def clear_screen():
    os.system('cls' if os.name == 'nt' else 'clear')

def print_tree(height=10):
    colors = ['\033[92m', '\033[1;32m', '\033[33m']  # Green, Bright Green, Yellow
    reset = '\033[0m'
    
    for i in range(height):
        spaces = ' ' * (height - i)
        stars = '*' * (2 * i + 1)
        color = colors[i % len(colors)]
        print(f"{spaces}{color}{stars}{reset}")
    
    # Trunk
    trunk_spaces = ' ' * (height - 1)
    print(f"{trunk_spaces}\033[33m||{reset}")
    print(f"{trunk_spaces}\033[33m||{reset}")
    
def animate_tree():
    height = 10
    clear_screen()
    
    for frame in range(20):
        clear_screen()
        
        # Twinkling effect
        colors = ['\033[92m', '\033[1;32m', '\033[33m', '\033[36m']
        
        print("\n" * 2)
        for i in range(height):
            spaces = ' ' * (height - i)
            stars = '*' * (2 * i + 1)
            color = colors[(i + frame) % len(colors)]
            print(f"{spaces}{color}{stars}\033[0m")
        
        # Trunk
        trunk_spaces = ' ' * (height - 1)
        print(f"{trunk_spaces}\033[33m||\033[0m")
        print(f"{trunk_spaces}\033[33m||\033[0m")
        
        print("\n" * 2)
        print("\033[1;31m🎄 Merry Christmas! 🎄\033[0m".center(40))
        
        time.sleep(0.3)

if __name__ == "__main__":
    animate_tree()