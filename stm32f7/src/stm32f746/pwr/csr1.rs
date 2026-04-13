///Register `CSR1` reader
pub type R = crate::R<CSR1rs>;
///Register `CSR1` writer
pub type W = crate::W<CSR1rs>;
///Field `WUIF` reader - Wakeup internal flag
pub type WUIF_R = crate::BitReader;
///Field `SBF` reader - Standby flag
pub type SBF_R = crate::BitReader;
///Field `PVDO` reader - PVD output
pub type PVDO_R = crate::BitReader;
///Field `BRR` reader - Backup regulator ready
pub type BRR_R = crate::BitReader;
///Field `BRE` reader - Backup regulator enable
pub type BRE_R = crate::BitReader;
///Field `BRE` writer - Backup regulator enable
pub type BRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VOSRDY` reader - Regulator voltage scaling output selection ready bit
pub type VOSRDY_R = crate::BitReader;
///Field `VOSRDY` writer - Regulator voltage scaling output selection ready bit
pub type VOSRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODRDY` reader - Over-drive mode ready
pub type ODRDY_R = crate::BitReader;
///Field `ODRDY` writer - Over-drive mode ready
pub type ODRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODSWRDY` reader - Over-drive mode switching ready
pub type ODSWRDY_R = crate::BitReader;
///Field `ODSWRDY` writer - Over-drive mode switching ready
pub type ODSWRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDRDY` reader - Under-drive ready flag
pub type UDRDY_R = crate::FieldReader;
///Field `UDRDY` writer - Under-drive ready flag
pub type UDRDY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Wakeup internal flag
    #[inline(always)]
    pub fn wuif(&self) -> WUIF_R {
        WUIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Standby flag
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PVD output
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Backup regulator ready
    #[inline(always)]
    pub fn brr(&self) -> BRR_R {
        BRR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 9 - Backup regulator enable
    #[inline(always)]
    pub fn bre(&self) -> BRE_R {
        BRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 14 - Regulator voltage scaling output selection ready bit
    #[inline(always)]
    pub fn vosrdy(&self) -> VOSRDY_R {
        VOSRDY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Over-drive mode ready
    #[inline(always)]
    pub fn odrdy(&self) -> ODRDY_R {
        ODRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Over-drive mode switching ready
    #[inline(always)]
    pub fn odswrdy(&self) -> ODSWRDY_R {
        ODSWRDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - Under-drive ready flag
    #[inline(always)]
    pub fn udrdy(&self) -> UDRDY_R {
        UDRDY_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR1")
            .field("wuif", &self.wuif())
            .field("sbf", &self.sbf())
            .field("pvdo", &self.pvdo())
            .field("brr", &self.brr())
            .field("bre", &self.bre())
            .field("vosrdy", &self.vosrdy())
            .field("odrdy", &self.odrdy())
            .field("odswrdy", &self.odswrdy())
            .field("udrdy", &self.udrdy())
            .finish()
    }
}
impl W {
    ///Bit 9 - Backup regulator enable
    #[inline(always)]
    pub fn bre(&mut self) -> BRE_W<'_, CSR1rs> {
        BRE_W::new(self, 9)
    }
    ///Bit 14 - Regulator voltage scaling output selection ready bit
    #[inline(always)]
    pub fn vosrdy(&mut self) -> VOSRDY_W<'_, CSR1rs> {
        VOSRDY_W::new(self, 14)
    }
    ///Bit 16 - Over-drive mode ready
    #[inline(always)]
    pub fn odrdy(&mut self) -> ODRDY_W<'_, CSR1rs> {
        ODRDY_W::new(self, 16)
    }
    ///Bit 17 - Over-drive mode switching ready
    #[inline(always)]
    pub fn odswrdy(&mut self) -> ODSWRDY_W<'_, CSR1rs> {
        ODSWRDY_W::new(self, 17)
    }
    ///Bits 18:19 - Under-drive ready flag
    #[inline(always)]
    pub fn udrdy(&mut self) -> UDRDY_W<'_, CSR1rs> {
        UDRDY_W::new(self, 18)
    }
}
/**power control/status register

You can [`read`](crate::Reg::read) this register and get [`csr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F746.html#PWR:CSR1)*/
pub struct CSR1rs;
impl crate::RegisterSpec for CSR1rs {
    type Ux = u32;
}
///`read()` method returns [`csr1::R`](R) reader structure
impl crate::Readable for CSR1rs {}
///`write(|w| ..)` method takes [`csr1::W`](W) writer structure
impl crate::Writable for CSR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR1 to value 0
impl crate::Resettable for CSR1rs {}
