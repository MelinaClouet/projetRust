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







function editNote(noteId , title , content){
    id.innerText=noteId;
    titleTag.value = title
    descTag.value = content
    addBox.click()

}

addBox.onclick = () => popupBox.classList.add("show");
closeBox.onclick = () => {
    titleTag.value= ''
    descTag.value= ''
    popupBox.classList.remove("show");

}

/*async function addNote() {
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


}*/

async function addNote() {
    // Récupérer les valeurs du formulaire
    const title = document.getElementById('title').value;
    const content = document.getElementById('description').value;
    console.log(id.innerText)
    if (!id.innerText){
        console.log('ici ADD');
        // Appeler la fonction back-end pour ajouter la nouvelle note
        try {
            const { invoke } = window.__TAURI__.tauri;

            // Appeler la commande back-end et récupérer les notes mises à jour
            const updatedNotes = await invoke('create_note_sqlite', { title: title, content: content });

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
        const idText = id.innerText;

// Transformation en entier
        const idInteger = parseInt(idText, 10);

        await invoke('update_note_sql', { id: idInteger, title: title, content: content });
        fetchAndShowNotes();
    }

    document.getElementById('title').value = '';
    document.getElementById('description').value = '';

    // Fermer la modal
    closeModal();

    fetchAndShowNotes();


}

async function fetchAndShowNotes() {
    try {
        const { invoke } = window.__TAURI__.tauri;

        // Call the backend command
        const notes = await invoke('get_notes');

        // Display notes in the UI
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
                <span>${note.content}</span>
            </div>
            
            <div class="bottom-content">
                <span>${note.date}</span>
               <div class="actions">
                    <button class="btnUpdate" onclick="editNote('${note.id}', '${note.title}', '${note.content}')">Modifier</button>

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


        console.log(index);

        // Appeler la commande backend pour supprimer la note avec l'ID donné
        await invoke('delete_note_sql', { id:index });

        // Rafraîchir l'affichage des notes après la suppression
        fetchAndShowNotes();
    } catch (error) {
        console.error('Erreur lors de la suppression de la note:', error);
    }
}



