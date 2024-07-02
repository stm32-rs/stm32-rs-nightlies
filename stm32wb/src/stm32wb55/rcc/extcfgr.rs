///Register `EXTCFGR` reader
pub type R = crate::R<EXTCFGRrs>;
///Register `EXTCFGR` writer
pub type W = crate::W<EXTCFGRrs>;
///Field `SHDHPRE` reader - Shared AHB prescaler
pub type SHDHPRE_R = crate::FieldReader;
///Field `SHDHPRE` writer - Shared AHB prescaler
pub type SHDHPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `C2HPRE` reader - CPU2 AHB prescaler
pub type C2HPRE_R = crate::FieldReader;
///Field `C2HPRE` writer - CPU2 AHB prescaler
pub type C2HPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SHDHPREF` reader - Shared AHB prescaler flag
pub type SHDHPREF_R = crate::BitReader;
///Field `C2HPREF` reader - CPU2 AHB prescaler flag
pub type C2HPREF_R = crate::BitReader;
///Field `RFCSS` reader - RF clock source selected
pub type RFCSS_R = crate::BitReader;
impl R {
    ///Bits 0:3 - Shared AHB prescaler
    #[inline(always)]
    pub fn shdhpre(&self) -> SHDHPRE_R {
        SHDHPRE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - CPU2 AHB prescaler
    #[inline(always)]
    pub fn c2hpre(&self) -> C2HPRE_R {
        C2HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 16 - Shared AHB prescaler flag
    #[inline(always)]
    pub fn shdhpref(&self) -> SHDHPREF_R {
        SHDHPREF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CPU2 AHB prescaler flag
    #[inline(always)]
    pub fn c2hpref(&self) -> C2HPREF_R {
        C2HPREF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - RF clock source selected
    #[inline(always)]
    pub fn rfcss(&self) -> RFCSS_R {
        RFCSS_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTCFGR")
            .field("rfcss", &self.rfcss())
            .field("c2hpref", &self.c2hpref())
            .field("shdhpref", &self.shdhpref())
            .field("c2hpre", &self.c2hpre())
            .field("shdhpre", &self.shdhpre())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Shared AHB prescaler
    #[inline(always)]
    #[must_use]
    pub fn shdhpre(&mut self) -> SHDHPRE_W<EXTCFGRrs> {
        SHDHPRE_W::new(self, 0)
    }
    ///Bits 4:7 - CPU2 AHB prescaler
    #[inline(always)]
    #[must_use]
    pub fn c2hpre(&mut self) -> C2HPRE_W<EXTCFGRrs> {
        C2HPRE_W::new(self, 4)
    }
}
/**Extended clock recovery register

You can [`read`](crate::Reg::read) this register and get [`extcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:EXTCFGR)*/
pub struct EXTCFGRrs;
impl crate::RegisterSpec for EXTCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`extcfgr::R`](R) reader structure
impl crate::Readable for EXTCFGRrs {}
///`write(|w| ..)` method takes [`extcfgr::W`](W) writer structure
impl crate::Writable for EXTCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EXTCFGR to value 0x0003_0000
impl crate::Resettable for EXTCFGRrs {
    const RESET_VALUE: u32 = 0x0003_0000;
}
