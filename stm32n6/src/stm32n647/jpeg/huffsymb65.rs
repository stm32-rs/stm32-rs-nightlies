///Register `HUFFSYMB65` reader
pub type R = crate::R<HUFFSYMB65rs>;
///Register `HUFFSYMB65` writer
pub type W = crate::W<HUFFSYMB65rs>;
///Field `DATA260` reader - Data 260
pub type DATA260_R = crate::FieldReader;
///Field `DATA260` writer - Data 260
pub type DATA260_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA261` reader - Data 261
pub type DATA261_R = crate::FieldReader;
///Field `DATA261` writer - Data 261
pub type DATA261_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA262` reader - Data 262
pub type DATA262_R = crate::FieldReader;
///Field `DATA262` writer - Data 262
pub type DATA262_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA263` reader - Data 263
pub type DATA263_R = crate::FieldReader;
///Field `DATA263` writer - Data 263
pub type DATA263_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 260
    #[inline(always)]
    pub fn data260(&self) -> DATA260_R {
        DATA260_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 261
    #[inline(always)]
    pub fn data261(&self) -> DATA261_R {
        DATA261_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 262
    #[inline(always)]
    pub fn data262(&self) -> DATA262_R {
        DATA262_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 263
    #[inline(always)]
    pub fn data263(&self) -> DATA263_R {
        DATA263_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB65")
            .field("data260", &self.data260())
            .field("data261", &self.data261())
            .field("data262", &self.data262())
            .field("data263", &self.data263())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 260
    #[inline(always)]
    pub fn data260(&mut self) -> DATA260_W<'_, HUFFSYMB65rs> {
        DATA260_W::new(self, 0)
    }
    ///Bits 8:15 - Data 261
    #[inline(always)]
    pub fn data261(&mut self) -> DATA261_W<'_, HUFFSYMB65rs> {
        DATA261_W::new(self, 8)
    }
    ///Bits 16:23 - Data 262
    #[inline(always)]
    pub fn data262(&mut self) -> DATA262_W<'_, HUFFSYMB65rs> {
        DATA262_W::new(self, 16)
    }
    ///Bits 24:31 - Data 263
    #[inline(always)]
    pub fn data263(&mut self) -> DATA263_W<'_, HUFFSYMB65rs> {
        DATA263_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb65::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb65::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFSYMB65)*/
pub struct HUFFSYMB65rs;
impl crate::RegisterSpec for HUFFSYMB65rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb65::R`](R) reader structure
impl crate::Readable for HUFFSYMB65rs {}
///`write(|w| ..)` method takes [`huffsymb65::W`](W) writer structure
impl crate::Writable for HUFFSYMB65rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB65 to value 0
impl crate::Resettable for HUFFSYMB65rs {}
