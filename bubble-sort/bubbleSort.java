import java.util.concurrent.ThreadLocalRandom;
import java.util.Arrays;

public class bubbleSort {
    public static void BubbleSort(int[] array){
        int n = array.length;
        
        for(int i = 0; i < n; i++){
            for(int j = 0; j < n - i - 1; j++){
                if(array[j] > array[j+1]){
                    int temp = array[j];
                    array[j] = array[j+1];
                    array[j+1] = temp;
                }
            }
        }
    }
    
    public static void main(String[] args) {
        int arraySize = ThreadLocalRandom.current().nextInt(1, 101);

        int[] tab = new int[arraySize];

        for(int k = 0; k < arraySize; k++){
            tab[k] = ThreadLocalRandom.current().nextInt(1, 101);
        }
        System.out.println("Przed sortowaniem:");
        System.out.println(Arrays.toString(tab));

        BubbleSort(tab);

        System.out.println("\nPo sortowaniu:");
        System.out.println(Arrays.toString(tab));
    }
    

}
