<template>
  <MilitaryGroupBox
    title="事件列表"
    class="event-list-container"
  >
    <div class="events military-scrollbar">
      <div
        v-for="event in events"
        :key="event.id"
        class="event-item"
        :class="[
          `event-item--${event.level}`,
          { 'event-item--active': selectedEvent === event.id },
        ]"
        @click="selectEvent(event.id)"
      >
        <div
          class="event-indicator"
          :class="`event-indicator--${event.level}`"
        />
        <div class="event-time text-monospace">{{ event.time }}</div>
        <div class="event-content">
          <div class="event-title text-subtitle">{{ event.title }}</div>
          <div class="event-description text-caption">
            {{ event.description }}
          </div>
        </div>
      </div>
    </div>
  </MilitaryGroupBox>
</template>

<script setup>
import { ref } from "vue";
import { useEvents } from "~/composables/useEvents";
import MilitaryGroupBox from "~/components/ui/MilitaryGroupBox.vue";

const { events } = useEvents();
const selectedEvent = ref(null);

const selectEvent = (eventId) => {
  selectedEvent.value = selectedEvent.value === eventId ? null : eventId;
};
</script>

<style scoped>
.event-list-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.events {
  flex: 1;
  margin: calc(-1 * var(--spacing-sm));
  padding: var(--spacing-sm);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
  overflow-y: auto;
}

.event-item {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-sm);
  padding: var(--spacing-xs) var(--spacing-sm);
  background: var(--tertiary-bg);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  border-left: 3px solid var(--border-color);
  cursor: pointer;
  transition: all var(--transition-fast);
  position: relative;
}

.event-item:hover {
  background: var(--panel-bg);
  border-color: var(--border-color-light);
  transform: translateX(2px);
}

.event-item--active {
  background: var(--panel-bg);
  border-color: var(--border-color-active);
  box-shadow: var(--glow-subtle);
}

.event-item--high {
  border-left-color: var(--danger-color);
}

.event-item--warning {
  border-left-color: var(--warning-color);
}

.event-item--normal {
  border-left-color: var(--success-color);
}

.event-item--low {
  border-left-color: var(--text-muted);
}

.event-indicator {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  margin-top: 2px;
  flex-shrink: 0;
  box-shadow: 0 0 4px currentColor;
}

.event-indicator--high {
  background: var(--danger-color);
  color: var(--danger-color);
}

.event-indicator--warning {
  background: var(--warning-color);
  color: var(--warning-color);
}

.event-indicator--normal {
  background: var(--success-color);
  color: var(--success-color);
}

.event-indicator--low {
  background: var(--text-muted);
  color: var(--text-muted);
}

.event-time {
  min-width: 50px;
  font-size: 9px;
  margin-top: 1px;
}

.event-content {
  flex: 1;
  min-width: 0;
}

.event-title {
  font-size: 11px;
  margin-bottom: 2px;
}

.event-description {
  font-size: 10px;
  line-height: 1.3;
}
</style>
