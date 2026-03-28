import { ref, onMounted, onUnmounted } from "vue";

const CLOSE_EVENT = "zenypst:close-context-menus";

export function useContextMenu() {
  const visible = ref(false);
  const x = ref(0);
  const y = ref(0);

  function close(): void {
    visible.value = false;
  }

  onMounted(() => { document.addEventListener(CLOSE_EVENT, close); });
  onUnmounted(() => { document.removeEventListener(CLOSE_EVENT, close); });

  function show(event: MouseEvent): void {
    event.preventDefault();
    event.stopPropagation();
    document.dispatchEvent(new Event(CLOSE_EVENT));
    x.value = event.clientX;
    y.value = event.clientY;
    setTimeout(() => { visible.value = true; }, 0);
  }

  return { visible, x, y, show, close };
}
