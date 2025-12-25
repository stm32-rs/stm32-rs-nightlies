///Register `HUFFSYMB61` reader
pub type R = crate::R<HUFFSYMB61rs>;
///Register `HUFFSYMB61` writer
pub type W = crate::W<HUFFSYMB61rs>;
///Field `DATA244` reader - Data 244
pub type DATA244_R = crate::FieldReader;
///Field `DATA244` writer - Data 244
pub type DATA244_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA245` reader - Data 245
pub type DATA245_R = crate::FieldReader;
///Field `DATA245` writer - Data 245
pub type DATA245_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA246` reader - Data 246
pub type DATA246_R = crate::FieldReader;
///Field `DATA246` writer - Data 246
pub type DATA246_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA247` reader - Data 247
pub type DATA247_R = crate::FieldReader;
///Field `DATA247` writer - Data 247
pub type DATA247_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 244
    #[inline(always)]
    pub fn data244(&self) -> DATA244_R {
        DATA244_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 245
    #[inline(always)]
    pub fn data245(&self) -> DATA245_R {
        DATA245_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 246
    #[inline(always)]
    pub fn data246(&self) -> DATA246_R {
        DATA246_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 247
    #[inline(always)]
    pub fn data247(&self) -> DATA247_R {
        DATA247_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB61")
            .field("data244", &self.data244())
            .field("data245", &self.data245())
            .field("data246", &self.data246())
            .field("data247", &self.data247())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 244
    #[inline(always)]
    pub fn data244(&mut self) -> DATA244_W<'_, HUFFSYMB61rs> {
        DATA244_W::new(self, 0)
    }
    ///Bits 8:15 - Data 245
    #[inline(always)]
    pub fn data245(&mut self) -> DATA245_W<'_, HUFFSYMB61rs> {
        DATA245_W::new(self, 8)
    }
    ///Bits 16:23 - Data 246
    #[inline(always)]
    pub fn data246(&mut self) -> DATA246_W<'_, HUFFSYMB61rs> {
        DATA246_W::new(self, 16)
    }
    ///Bits 24:31 - Data 247
    #[inline(always)]
    pub fn data247(&mut self) -> DATA247_W<'_, HUFFSYMB61rs> {
        DATA247_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb61::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb61::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFSYMB61)*/
pub struct HUFFSYMB61rs;
impl crate::RegisterSpec for HUFFSYMB61rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb61::R`](R) reader structure
impl crate::Readable for HUFFSYMB61rs {}
///`write(|w| ..)` method takes [`huffsymb61::W`](W) writer structure
impl crate::Writable for HUFFSYMB61rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB61 to value 0
impl crate::Resettable for HUFFSYMB61rs {}
