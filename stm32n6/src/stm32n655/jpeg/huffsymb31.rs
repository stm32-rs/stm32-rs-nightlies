///Register `HUFFSYMB31` reader
pub type R = crate::R<HUFFSYMB31rs>;
///Register `HUFFSYMB31` writer
pub type W = crate::W<HUFFSYMB31rs>;
///Field `DATA124` reader - Data 124
pub type DATA124_R = crate::FieldReader;
///Field `DATA124` writer - Data 124
pub type DATA124_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA125` reader - Data 125
pub type DATA125_R = crate::FieldReader;
///Field `DATA125` writer - Data 125
pub type DATA125_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA126` reader - Data 126
pub type DATA126_R = crate::FieldReader;
///Field `DATA126` writer - Data 126
pub type DATA126_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA127` reader - Data 127
pub type DATA127_R = crate::FieldReader;
///Field `DATA127` writer - Data 127
pub type DATA127_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 124
    #[inline(always)]
    pub fn data124(&self) -> DATA124_R {
        DATA124_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 125
    #[inline(always)]
    pub fn data125(&self) -> DATA125_R {
        DATA125_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 126
    #[inline(always)]
    pub fn data126(&self) -> DATA126_R {
        DATA126_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 127
    #[inline(always)]
    pub fn data127(&self) -> DATA127_R {
        DATA127_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB31")
            .field("data124", &self.data124())
            .field("data125", &self.data125())
            .field("data126", &self.data126())
            .field("data127", &self.data127())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 124
    #[inline(always)]
    pub fn data124(&mut self) -> DATA124_W<'_, HUFFSYMB31rs> {
        DATA124_W::new(self, 0)
    }
    ///Bits 8:15 - Data 125
    #[inline(always)]
    pub fn data125(&mut self) -> DATA125_W<'_, HUFFSYMB31rs> {
        DATA125_W::new(self, 8)
    }
    ///Bits 16:23 - Data 126
    #[inline(always)]
    pub fn data126(&mut self) -> DATA126_W<'_, HUFFSYMB31rs> {
        DATA126_W::new(self, 16)
    }
    ///Bits 24:31 - Data 127
    #[inline(always)]
    pub fn data127(&mut self) -> DATA127_W<'_, HUFFSYMB31rs> {
        DATA127_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB31)*/
pub struct HUFFSYMB31rs;
impl crate::RegisterSpec for HUFFSYMB31rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb31::R`](R) reader structure
impl crate::Readable for HUFFSYMB31rs {}
///`write(|w| ..)` method takes [`huffsymb31::W`](W) writer structure
impl crate::Writable for HUFFSYMB31rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB31 to value 0
impl crate::Resettable for HUFFSYMB31rs {}
