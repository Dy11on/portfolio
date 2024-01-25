<script>
  import Step from "./Step.svelte";
  import Timeline from "./Timeline.svelte";

  let steps = [
    {name: 'Portfolio', icon: 'fa-solid fa-user', href: 'https://github.com/Dy11on/portfolio'},
    {name: 'Rust app', icon: 'fa-brands fa-rust', href: 'https://github.com/Dy11on/actix-microservice'},
    {name: 'Permission Based Project', icon: 'fa-solid fa-shield-halved', href:'https://github.com/Dy11on/Permission-Based-Two-Factor'},
  ]


</script>

<main class="flex flex-col flex-1 p-4">
  <!-- delete everything past grid for it to be more centered? -->
  <section id="introPage" class="grid grid-cols-1 lg:grid-cols-2 gap-10 py-8 sm:py-14">
    <div class="flex flex-col lg:justify-center text-center gap-6 md:gap-8 lg:gap-10">
      <h2 class="font-semibold text-4xl sm:text-5xl md:text-5xl">
        Hi I'm <span class="text-violet-400">Dyllon</span> Ngmon <br/>
      </h2>

      <a class="blueShadow mx-auto lg:mr-auto text-base sm:text-lg md:text-xl relative overflow-hidden px-6 px-3 group rounded-full bg-white text-slate-950" 
      href="mailto:dngmonwork@gmail.com"
      >
        <div class="absolute top-0 right-full w-full h-full bg-violet-400 opacity-20 group-hover:translate-x-full z-0 duration-200"/>
        <h4 class="relative z-9">Get in touch!&rarr;</h4>
      </a>
    </div>
    <div class="relative shadow-2xl grid place-items-center">
      <img src={"images/me.jpg"} alt="Portfolio Image" class="image object-cover z-[2] max-h-[30vh]"/>
    </div>

  </section>


  <section id="about" class="py-20 pt-10 lg:pt-16 lg:py-32 flex flex-col gap-16 sm:gap-20 md:gap-24 relative">
    <div class="flex flex-col gap-2 text-center relative before:absolute before:top-0 before:left-0 before:w-2/3 
    before:h-1.5 before:bg-violet-700 after:absolute after:bottom-0 after:right-0 after:w-2/3 after:h-1.5 after:bg-violet-700 py-4">

    <h3 class="font-semibold text-3xl sm:text-4xl md:text-5xl">
      About me
    </h3> 
    <div class="flex flex-col gap-20 w-full mx-auto max-w-[800px]"/>
    <p>Hi! My name is Dyllon and I enjoy tinkering with different types of technology.  I started out in cybersecurity but discovered a passion for coding as I continued
	    with my education.  I pivoted into software engineering after graduating with my bachelors in cybersecurity and doing a couple of cybersecurity related internships.<br><br> 
 
	    I have worked on fullstack applications at Wave Financial, leveraging the React and Django frameworks.  I also worked on microservices at Canadian Tire using Spring Boot, Kubernetes, Terraform and Microsoft Azure.  In my free time I enjoy learning new technologies.  This <a class="projects" href="#top">portfolio</a> was created using SvelteKit and Bun.  I have created a <a class="projects" href="https://github.com/Dy11on/actix-microservice">microservice</a> in Rust,  and I am currently working on a 2D game in Rust using
	    a custom game engine!</p>

    </div>
  </section>
  <section id="experience">
    <Timeline/>

  </section>

  <section id="projects" class="py-20 lg:py-32 flex flex-col gap-24">
    <div class="flex flex-col gap-2 text-center">
      <h3 class="font-semibold text-3xl sm:text-4xl md:text-5xl">
        Some of my projects
      </h3>
    </div>
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-12 lg:gap-10">
      <Step step={steps[0]}>
        <p>My portfolio that was made with svelte, bun, and docker.</p>
      </Step>
      <Step step={steps[1]}>
        <p>Simple Rust API that was built using the Axum framework,dockerized, and deployed to google cloud run.</p>
      </Step>
      <Step step={steps[2]}>
        <p>Permissions based application created using golang and react native</p>
      </Step>
    </div>
  </section>

</main> 

