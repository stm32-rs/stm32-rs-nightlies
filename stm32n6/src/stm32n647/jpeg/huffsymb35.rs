///Register `HUFFSYMB35` reader
pub type R = crate::R<HUFFSYMB35rs>;
///Register `HUFFSYMB35` writer
pub type W = crate::W<HUFFSYMB35rs>;
///Field `DATA140` reader - Data 140
pub type DATA140_R = crate::FieldReader;
///Field `DATA140` writer - Data 140
pub type DATA140_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA141` reader - Data 141
pub type DATA141_R = crate::FieldReader;
///Field `DATA141` writer - Data 141
pub type DATA141_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA142` reader - Data 142
pub type DATA142_R = crate::FieldReader;
///Field `DATA142` writer - Data 142
pub type DATA142_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA143` reader - Data 143
pub type DATA143_R = crate::FieldReader;
///Field `DATA143` writer - Data 143
pub type DATA143_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 140
    #[inline(always)]
    pub fn data140(&self) -> DATA140_R {
        DATA140_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 141
    #[inline(always)]
    pub fn data141(&self) -> DATA141_R {
        DATA141_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 142
    #[inline(always)]
    pub fn data142(&self) -> DATA142_R {
        DATA142_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 143
    #[inline(always)]
    pub fn data143(&self) -> DATA143_R {
        DATA143_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB35")
            .field("data140", &self.data140())
            .field("data141", &self.data141())
            .field("data142", &self.data142())
            .field("data143", &self.data143())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 140
    #[inline(always)]
    pub fn data140(&mut self) -> DATA140_W<'_, HUFFSYMB35rs> {
        DATA140_W::new(self, 0)
    }
    ///Bits 8:15 - Data 141
    #[inline(always)]
    pub fn data141(&mut self) -> DATA141_W<'_, HUFFSYMB35rs> {
        DATA141_W::new(self, 8)
    }
    ///Bits 16:23 - Data 142
    #[inline(always)]
    pub fn data142(&mut self) -> DATA142_W<'_, HUFFSYMB35rs> {
        DATA142_W::new(self, 16)
    }
    ///Bits 24:31 - Data 143
    #[inline(always)]
    pub fn data143(&mut self) -> DATA143_W<'_, HUFFSYMB35rs> {
        DATA143_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb35::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb35::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFSYMB35)*/
pub struct HUFFSYMB35rs;
impl crate::RegisterSpec for HUFFSYMB35rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb35::R`](R) reader structure
impl crate::Readable for HUFFSYMB35rs {}
///`write(|w| ..)` method takes [`huffsymb35::W`](W) writer structure
impl crate::Writable for HUFFSYMB35rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB35 to value 0
impl crate::Resettable for HUFFSYMB35rs {}
