///Register `HUFFSYMB22` reader
pub type R = crate::R<HUFFSYMB22rs>;
///Register `HUFFSYMB22` writer
pub type W = crate::W<HUFFSYMB22rs>;
///Field `DATA88` reader - Data 88
pub type DATA88_R = crate::FieldReader;
///Field `DATA88` writer - Data 88
pub type DATA88_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA89` reader - Data 89
pub type DATA89_R = crate::FieldReader;
///Field `DATA89` writer - Data 89
pub type DATA89_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA90` reader - Data 90
pub type DATA90_R = crate::FieldReader;
///Field `DATA90` writer - Data 90
pub type DATA90_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA91` reader - Data 91
pub type DATA91_R = crate::FieldReader;
///Field `DATA91` writer - Data 91
pub type DATA91_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 88
    #[inline(always)]
    pub fn data88(&self) -> DATA88_R {
        DATA88_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 89
    #[inline(always)]
    pub fn data89(&self) -> DATA89_R {
        DATA89_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 90
    #[inline(always)]
    pub fn data90(&self) -> DATA90_R {
        DATA90_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 91
    #[inline(always)]
    pub fn data91(&self) -> DATA91_R {
        DATA91_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB22")
            .field("data88", &self.data88())
            .field("data89", &self.data89())
            .field("data90", &self.data90())
            .field("data91", &self.data91())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 88
    #[inline(always)]
    pub fn data88(&mut self) -> DATA88_W<'_, HUFFSYMB22rs> {
        DATA88_W::new(self, 0)
    }
    ///Bits 8:15 - Data 89
    #[inline(always)]
    pub fn data89(&mut self) -> DATA89_W<'_, HUFFSYMB22rs> {
        DATA89_W::new(self, 8)
    }
    ///Bits 16:23 - Data 90
    #[inline(always)]
    pub fn data90(&mut self) -> DATA90_W<'_, HUFFSYMB22rs> {
        DATA90_W::new(self, 16)
    }
    ///Bits 24:31 - Data 91
    #[inline(always)]
    pub fn data91(&mut self) -> DATA91_W<'_, HUFFSYMB22rs> {
        DATA91_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB22)*/
pub struct HUFFSYMB22rs;
impl crate::RegisterSpec for HUFFSYMB22rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb22::R`](R) reader structure
impl crate::Readable for HUFFSYMB22rs {}
///`write(|w| ..)` method takes [`huffsymb22::W`](W) writer structure
impl crate::Writable for HUFFSYMB22rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB22 to value 0
impl crate::Resettable for HUFFSYMB22rs {}
