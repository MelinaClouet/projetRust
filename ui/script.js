document.addEventListener('DOMContentLoaded', () => {
    fetchAndShowNotes();




});


const addBox = document.querySelector(".add-box")
const popupBox = document.querySelector(".popup-box")


const titleTag = popupBox.querySelector("input")
const descTag = popupBox.querySelector("textarea")
const addBtn = popupBox.querySelector("button")

const id=document.getElementById("id");
const menuel = document.querySelector('.iconel')







function editNote(noteId , title , description){
    id.innerText=noteId;
    titleTag.value = title
    descTag.value = description
    addBox.click()

}

addBox.onclick = () => popupBox.classList.add("show");
closeBox.onclick = () => {
    titleTag.value= ''
    descTag.value= ''
    popupBox.classList.remove("show");

}

async function addNote() {
    // Récupérer les valeurs du formulaire
    const title = document.getElementById('title').value;
    const description = document.getElementById('description').value;
    console.log(id.innerText)
    if (!id.innerText){
        console.log('ici ADD');
        // Appeler la fonction back-end pour ajouter la nouvelle note
        try {
            const { invoke } = window.__TAURI__.tauri;

            // Appeler la commande back-end et récupérer les notes mises à jour
            const updatedNotes = await invoke('create_note', { title: title, description: description });

            // Afficher les notes mises à jour dans la page HTML
            showNotes(updatedNotes);
        } catch (error) {
            console.error('Erreur lors de l\'ajout de la note:', error);
        }
    }
    else{
        console.log('ici UPD')
        const { invoke } = window.__TAURI__.tauri;

        // Appeler la commande back-end et récupérer les notes mises à jour
         await invoke('update_note', { id: id.innerText, newTitle: title, newDescription: description });
    }


}

async function fetchAndShowNotes() {
    try {
        const { invoke } = window.__TAURI__.tauri;

        // Appeler la commande backend pour récupérer toutes les notes
        const notes = await invoke('fetch_notes');

        // Afficher les notes récupérées dans l'interface utilisateur
        showNotes(notes);
    } catch (error) {
        console.error('Erreur lors de la récupération des notes:', error);
    }
}


function showNotes(notes) {
    const notesContainer = document.getElementById('notes-container');
    notesContainer.innerHTML = ''; // Effacer le contenu précédent

    // Ajouter les nouvelles notes dans la page HTML
    notes.forEach((note, index) => {
        let litag = `<li class="note" >
            <div class="details">
                <p>${note.title}</p>
                <span>${note.description}</span>
            </div>
            
            <div class="bottom-content">
                <span>${note.date}</span>
               <div class="actions">
                    <button class="btnUpdate" onclick="editNote('${note.id}', '${note.title}', '${note.description}')">Modifier</button>

                    <button class="btnDelete" onclick="deleteNote(${note.id})">Supprimer</button>
               </div>
            </div>

        </li>`;

        notesContainer.insertAdjacentHTML('beforeend', litag);
    });
}

function closeModal() {
    popupBox.classList.remove("show");
}


async function deleteNote(index) {
    try {
        const { invoke } = window.__TAURI__.tauri;

        // Convertir l'index en chaîne de caractères
        const id = index.toString();

        // Appeler la commande backend pour supprimer la note avec l'ID donné
        await invoke('delete_note', { id: id });

        // Rafraîchir l'affichage des notes après la suppression
        fetchAndShowNotes();
    } catch (error) {
        console.error('Erreur lors de la suppression de la note:', error);
    }
}
