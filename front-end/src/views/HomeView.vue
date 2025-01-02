<script setup lang="ts">
import InputText from 'primevue/inputtext';
import Button from 'primevue/button';
import IftaLabel from 'primevue/iftalabel';


import { ref } from 'vue';

const value = ref(null);
</script>

<template>
  <div class="home-page">
    <header class="header">
      <nav class="navbar">
        <div><img src="./../images/logo.svg" class="logo"></div>
        <ul class="nav-links">
          <li><a href="#">METIERS</a></li>
          <li><a href="#">FORMATIONS</a></li>
          <li><a href="#">NOS SERVICES</a></li>
          <li><a href="#">DECOUVRIR</a></li>
        </ul>
      </nav>
      <div class="flex flex-row justify-around items-center gap-2">
        <div>
          <div class="flex gap-2 flex-col m-3">
            <div class="flex ">
              <p class="text-3xl">Un accompagnement</p>
            </div>
            <div class="flex flex-row gap-2">
              <div class="flex flex-col  text-3xl">
                <p class="font-bold ">personnalisé </p><img src="./../images/trait_orange.svg"
                  class="rotate-180 w-15 h-2 ">
              </div>
              <p class="text-3xl">pour</p>
            </div>
            <div class="flex flex-row gap-2">
              <p class="text-3xl"> l'orientation des
              <p class="font-bold text-3xl"> jeunes </p>
              </p>
            </div>

          </div>
          <div class="flex flex-row gap-3">
            <IftaLabel>
              <InputText id="metier" v-model="value" placeholder="Ingénieur" variant="filled" />
              <label for="username">Métier, certification</label>
            </IftaLabel>
            <IftaLabel>
              <InputText id="lieu" v-model="value" placeholder="Paris" variant="filled" />
              <label for="username">Où ?</label>
            </IftaLabel>
            <Button icon="pi pi-search" severity="warn"></Button>
          </div>
        </div>
        <div>
          <img src="./../images/student.png">
        </div>
      </div>

    </header>
    <section class="who-are-we">
      <div class="flex flex-row items-center justify-center gap-1 m-2">
        <p class="qui-nous-sommes-text text-5xl ">Qui sommes </p>
        <div class="flex flex-col gap-2">
          <p class="qui-nous-sommes-text text-5xl ml-1 mt-4">nous</p>
          <img src="./../images/trait_orange.svg" class=" w-25 h-2  ">
        </div>
        <p class="qui-nous-sommes-text text-5xl">?</p>
      </div>
      <p class="m-4">Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce quis vestibulum urna. Etiam consequat tortor ut
        lacus feugiat efficitur. Suspendisse interdum dui at nibh mattis vehicula. 
      </p>
      <div class="flex flex-row justify-around gap-10">
        <div v-for="i in 3" class="flex flex-col items-center gap-1">
          <img src="./../images/icons-search.svg" class="h-10 w-10">
          <p class="font-bold">Qui sommes nous ?</p>
          <p class="text-xs">Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam.</p>
          <p class="text-xs text-orange-600"> Découvrir la suite</p>
        </div>
      </div>
    </section>

    <section class="trainings">
      <h2>Nos formations</h2>
      <div class="grid grid-cols-2 gap-5">
        <div v-for="i in 4">
          <div class="m-5 relative">
            <img src="./../images/image_pack.png">
            <div class="absolute inset-x-0 bottom-0 h-20  border-solid border-blue-700 border-2 "> Pack essentiel</div>
          </div>
        </div>
      </div>
    </section>

    <section class="testimonials">
      <h2>Fiches Métiers</h2>
      <div class="testimonial-cards">
        <div class="testimonial-card">
          <img alt="Testimonial 1">
          <p>"Description du témoignage 1."</p>
        </div>
        <div class="testimonial-card">
          <img alt="Testimonial 2">
          <p>"Description du témoignage 2."</p>
        </div>
        <div class="testimonial-card">
          <img alt="Testimonial 3">
          <p>"Description du témoignage 3."</p>
        </div>
        <div class="testimonial-card">
          <img alt="Testimonial 4">
          <p>"Description du témoignage 4."</p>
        </div>
      </div>
    </section>

    <footer class="footer">
      <p>&copy; 2023 Votre Entreprise. Tous droits réservés.</p>
    </footer>
  </div>
</template>



<style scoped>
.home-page {
  /* font-family: Arial, sans-serif; */
}

.logo {
  width: 150px;
}

.header {
  background-color: #2F4B98;
  color: white;
  padding: 20px;
  text-align: center;

}

.qui-nous-sommes-text {
  color: #2F4B98;
  font-weight: bold;
}

.navbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.nav-links {
  list-style: none;
  display: flex;
  gap: 20px;
}

.nav-links a {
  color: white;
  text-decoration: none;
}

.hero-section {
  margin-top: 40px;
}

.hero-section h1 {
  font-size: 2.5em;
  margin-bottom: 20px;
}

.cta-button {
  background-color: #ffa502;
  color: white;
  border: none;
  padding: 10px 20px;
  font-size: 1em;
  cursor: pointer;
}

.who-are-we,





.feature,
.training-card,
.testimonial-card {
  background-color: #f9f9f9;
  padding: 20px;
  border-radius: 8px;
  text-align: left;
}

.training-card img,
.testimonial-card img {
  max-width: 100%;
  height: auto;
  border-radius: 8px;
}

.footer {
  background-color: #333;
  color: white;
  text-align: center;
  padding: 10px 0;
  position: fixed;
  width: 100%;
  bottom: 0;
}
</style>