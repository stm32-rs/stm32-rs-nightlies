///Register `TSCSDIF_CR` reader
pub type R = crate::R<TSCSDIF_CRrs>;
///Register `TSCSDIF_CR` writer
pub type W = crate::W<TSCSDIF_CRrs>;
///Field `SDIF_WDATA` reader - Serial interface write data
pub type SDIF_WDATA_R = crate::FieldReader<u32>;
///Field `SDIF_WDATA` writer - Serial interface write data
pub type SDIF_WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
///Field `SDIF_ADDR` reader - Serial interface register address
pub type SDIF_ADDR_R = crate::FieldReader;
///Field `SDIF_ADDR` writer - Serial interface register address
pub type SDIF_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SDIF_WRN` reader - Serial interface write/no read control bit
pub type SDIF_WRN_R = crate::BitReader;
///Field `SDIF_WRN` writer - Serial interface write/no read control bit
pub type SDIF_WRN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIF_PROG` writer - Serial interface program request
pub type SDIF_PROG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:23 - Serial interface write data
    #[inline(always)]
    pub fn sdif_wdata(&self) -> SDIF_WDATA_R {
        SDIF_WDATA_R::new(self.bits & 0x00ff_ffff)
    }
    ///Bits 24:26 - Serial interface register address
    #[inline(always)]
    pub fn sdif_addr(&self) -> SDIF_ADDR_R {
        SDIF_ADDR_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bit 27 - Serial interface write/no read control bit
    #[inline(always)]
    pub fn sdif_wrn(&self) -> SDIF_WRN_R {
        SDIF_WRN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSCSDIF_CR")
            .field("sdif_wdata", &self.sdif_wdata())
            .field("sdif_addr", &self.sdif_addr())
            .field("sdif_wrn", &self.sdif_wrn())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - Serial interface write data
    #[inline(always)]
    pub fn sdif_wdata(&mut self) -> SDIF_WDATA_W<'_, TSCSDIF_CRrs> {
        SDIF_WDATA_W::new(self, 0)
    }
    ///Bits 24:26 - Serial interface register address
    #[inline(always)]
    pub fn sdif_addr(&mut self) -> SDIF_ADDR_W<'_, TSCSDIF_CRrs> {
        SDIF_ADDR_W::new(self, 24)
    }
    ///Bit 27 - Serial interface write/no read control bit
    #[inline(always)]
    pub fn sdif_wrn(&mut self) -> SDIF_WRN_W<'_, TSCSDIF_CRrs> {
        SDIF_WRN_W::new(self, 27)
    }
    ///Bit 31 - Serial interface program request
    #[inline(always)]
    pub fn sdif_prog(&mut self) -> SDIF_PROG_W<'_, TSCSDIF_CRrs> {
        SDIF_PROG_W::new(self, 31)
    }
}
/**DTS TSC SDIF register

You can [`read`](crate::Reg::read) this register and get [`tscsdif_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscsdif_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TSCSDIF_CR)*/
pub struct TSCSDIF_CRrs;
impl crate::RegisterSpec for TSCSDIF_CRrs {
    type Ux = u32;
}
///`read()` method returns [`tscsdif_cr::R`](R) reader structure
impl crate::Readable for TSCSDIF_CRrs {}
///`write(|w| ..)` method takes [`tscsdif_cr::W`](W) writer structure
impl crate::Writable for TSCSDIF_CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TSCSDIF_CR to value 0
impl crate::Resettable for TSCSDIF_CRrs {}
