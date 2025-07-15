<template>
    <div class="parent-dashboard">
        <h1>Tableau de bord Parent</h1>
        <section class="overview">
            <h2>Vue d'ensemble</h2>
            <ul>
                <li>Enfants inscrits : {{ children.length }}</li>
                <li>Messages non lus : {{ unreadMessages }}</li>
                <li>Prochain événement : {{ nextEvent }}</li>
            </ul>
        </section>

        <section class="children-list">
            <h2>Mes enfants</h2>
            <ul>
                <li v-for="child in children" :key="child.id">
                    {{ child.name }} - Classe : {{ child.class }}
                </li>
            </ul>
        </section>

        <section class="messages">
            <h2>Messages récents</h2>
            <ul>
                <li v-for="msg in messages" :key="msg.id">
                    <strong>{{ msg.sender }}:</strong> {{ msg.content }}
                </li>
            </ul>
        </section>
    </div>
</template>

<script>
export default {
    name: "ParentDashboard",
    data() {
        return {
            children: [
                { id: 1, name: "Alice Dupont", class: "CM1" },
                { id: 2, name: "Lucas Dupont", class: "CE2" }
            ],
            messages: [
                { id: 1, sender: "Professeur", content: "Réunion parents-professeurs vendredi." },
                { id: 2, sender: "Administration", content: "Rappel : paiement cantine avant le 10." }
            ],
            unreadMessages: 2,
            nextEvent: "Sortie scolaire le 15 juin"
        };
    }
};
</script>

<style scoped>
.parent-dashboard {
    max-width: 700px;
    margin: 0 auto;
    padding: 2rem;
    background: #f9f9f9;
    border-radius: 8px;
}
h1, h2 {
    color: #2c3e50;
}
section {
    margin-bottom: 2rem;
}
ul {
    list-style: none;
    padding: 0;
}
li {
    margin-bottom: 0.5rem;
}
</style>