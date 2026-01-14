///Register `HUFFSYMB6` reader
pub type R = crate::R<HUFFSYMB6rs>;
///Register `HUFFSYMB6` writer
pub type W = crate::W<HUFFSYMB6rs>;
///Field `DATA24` reader - Data 24
pub type DATA24_R = crate::FieldReader;
///Field `DATA24` writer - Data 24
pub type DATA24_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA25` reader - Data 25
pub type DATA25_R = crate::FieldReader;
///Field `DATA25` writer - Data 25
pub type DATA25_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA26` reader - Data 26
pub type DATA26_R = crate::FieldReader;
///Field `DATA26` writer - Data 26
pub type DATA26_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA27` reader - Data 27
pub type DATA27_R = crate::FieldReader;
///Field `DATA27` writer - Data 27
pub type DATA27_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 24
    #[inline(always)]
    pub fn data24(&self) -> DATA24_R {
        DATA24_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 25
    #[inline(always)]
    pub fn data25(&self) -> DATA25_R {
        DATA25_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 26
    #[inline(always)]
    pub fn data26(&self) -> DATA26_R {
        DATA26_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 27
    #[inline(always)]
    pub fn data27(&self) -> DATA27_R {
        DATA27_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB6")
            .field("data24", &self.data24())
            .field("data25", &self.data25())
            .field("data26", &self.data26())
            .field("data27", &self.data27())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 24
    #[inline(always)]
    pub fn data24(&mut self) -> DATA24_W<'_, HUFFSYMB6rs> {
        DATA24_W::new(self, 0)
    }
    ///Bits 8:15 - Data 25
    #[inline(always)]
    pub fn data25(&mut self) -> DATA25_W<'_, HUFFSYMB6rs> {
        DATA25_W::new(self, 8)
    }
    ///Bits 16:23 - Data 26
    #[inline(always)]
    pub fn data26(&mut self) -> DATA26_W<'_, HUFFSYMB6rs> {
        DATA26_W::new(self, 16)
    }
    ///Bits 24:31 - Data 27
    #[inline(always)]
    pub fn data27(&mut self) -> DATA27_W<'_, HUFFSYMB6rs> {
        DATA27_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFSYMB6)*/
pub struct HUFFSYMB6rs;
impl crate::RegisterSpec for HUFFSYMB6rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb6::R`](R) reader structure
impl crate::Readable for HUFFSYMB6rs {}
///`write(|w| ..)` method takes [`huffsymb6::W`](W) writer structure
impl crate::Writable for HUFFSYMB6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB6 to value 0
impl crate::Resettable for HUFFSYMB6rs {}
