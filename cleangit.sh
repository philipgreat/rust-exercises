

function deletehistory(){

    target=$1
    git add .
    git commit -m"commit for removing extra files"
    git pull
    git push
    git filter-branch --force --index-filter 'git rm -r —cached --ignore-unmatch todo-cli/meilisearch' --prune-empty --tag-name-filter cat -- --all
    git commit -m"remove  extra class"
    git push origin --force --all

}
function test(){

    name=$1
    echo ${name}

}

function listfolder(){

    
    for d in */ ; do
        if [ -d "$d/target" ]; then
            echo "${d}target"
        fi
        
    done


}

function deletetarget(){

    git add .
    git commit -m"commit for removing extra files"
    git pull
    git push
    for d in */ ; do
        if [ -d "$d/target" ]; then
            targetfolder="${d}target"
            echo "git rm -r —cached --ignore-unmatch ${targetfolder}"
            git filter-branch --force --index-filter "git rm -r —cached --ignore-unmatch ${targetfolder}" --prune-empty --tag-name-filter cat -- --all
        fi
        
    done
    git commit -m"remove  extra class"
    git push origin --force --all

}



deletehistory 
