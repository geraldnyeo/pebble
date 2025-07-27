export function clickOutside(node: any) {
    // use action for this because document listener should only be in effect for component lifecycle

    const handleClick = (event: any) => {
        if (node && !node.contains(event.target) && !event.defaultPrevented) {
            node.dispatchEvent(
                new CustomEvent('click_outside', node)
            )
        }
    }

    document.addEventListener('click', handleClick, true); // add listener at document level, not node, to listen for clicks outside node

    return {
        destroy() {
            document.removeEventListener('click', handleClick, true);
        }
    }
}