///Register `DCR` reader
pub type R = crate::R<DCRrs>;
///Register `DCR` writer
pub type W = crate::W<DCRrs>;
///Field `CKMODE` reader - CKMODE
pub type CKMODE_R = crate::BitReader;
///Field `CKMODE` writer - CKMODE
pub type CKMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSHT` reader - CSHT
pub type CSHT_R = crate::FieldReader;
///Field `CSHT` writer - CSHT
pub type CSHT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FSIZE` reader - FSIZE
pub type FSIZE_R = crate::FieldReader;
///Field `FSIZE` writer - FSIZE
pub type FSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 0 - CKMODE
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:10 - CSHT
    #[inline(always)]
    pub fn csht(&self) -> CSHT_R {
        CSHT_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 16:20 - FSIZE
    #[inline(always)]
    pub fn fsize(&self) -> FSIZE_R {
        FSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCR")
            .field("ckmode", &self.ckmode())
            .field("csht", &self.csht())
            .field("fsize", &self.fsize())
            .finish()
    }
}
impl W {
    ///Bit 0 - CKMODE
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W<'_, DCRrs> {
        CKMODE_W::new(self, 0)
    }
    ///Bits 8:10 - CSHT
    #[inline(always)]
    pub fn csht(&mut self) -> CSHT_W<'_, DCRrs> {
        CSHT_W::new(self, 8)
    }
    ///Bits 16:20 - FSIZE
    #[inline(always)]
    pub fn fsize(&mut self) -> FSIZE_W<'_, DCRrs> {
        FSIZE_W::new(self, 16)
    }
}
/**QUADSPI device configuration register

You can [`read`](crate::Reg::read) this register and get [`dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:DCR)*/
pub struct DCRrs;
impl crate::RegisterSpec for DCRrs {
    type Ux = u32;
}
///`read()` method returns [`dcr::R`](R) reader structure
impl crate::Readable for DCRrs {}
///`write(|w| ..)` method takes [`dcr::W`](W) writer structure
impl crate::Writable for DCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCR to value 0
impl crate::Resettable for DCRrs {}
