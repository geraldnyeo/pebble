export function rightClick(node: any) {

    const handleRightClick = (event: any) => {
        event.preventDefault();
        if (node && node.parentNode.contains(event.target)) {
            node.dispatchEvent(
                new CustomEvent('right_click', { detail: {
                    "clientX": event.clientX,
                    "clientY": event.clientY,
                } })
            )
        } else {
            node.dispatchEvent(
                new CustomEvent('false_right_click')
            )
        }
    }

    document.addEventListener('contextmenu', handleRightClick, true); // add listener at document level, not node, to listen for clicks outside node

    return {
        destroy() {
            document.removeEventListener('contextmenu', handleRightClick, true);
        }
    }
}