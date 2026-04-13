///Register `RIMC_ATTR11` reader
pub type R = crate::R<RIMC_ATTR11rs>;
///Register `RIMC_ATTR11` writer
pub type W = crate::W<RIMC_ATTR11rs>;
///Field `MCID` reader - master CID
pub type MCID_R = crate::FieldReader;
///Field `MCID` writer - master CID
pub type MCID_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MSEC` reader - master secure
pub type MSEC_R = crate::BitReader;
///Field `MSEC` writer - master secure
pub type MSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPRIV` reader - master privileged
pub type MPRIV_R = crate::BitReader;
///Field `MPRIV` writer - master privileged
pub type MPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 4:6 - master CID
    #[inline(always)]
    pub fn mcid(&self) -> MCID_R {
        MCID_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 8 - master secure
    #[inline(always)]
    pub fn msec(&self) -> MSEC_R {
        MSEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - master privileged
    #[inline(always)]
    pub fn mpriv(&self) -> MPRIV_R {
        MPRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RIMC_ATTR11")
            .field("mcid", &self.mcid())
            .field("msec", &self.msec())
            .field("mpriv", &self.mpriv())
            .finish()
    }
}
impl W {
    ///Bits 4:6 - master CID
    #[inline(always)]
    pub fn mcid(&mut self) -> MCID_W<'_, RIMC_ATTR11rs> {
        MCID_W::new(self, 4)
    }
    ///Bit 8 - master secure
    #[inline(always)]
    pub fn msec(&mut self) -> MSEC_W<'_, RIMC_ATTR11rs> {
        MSEC_W::new(self, 8)
    }
    ///Bit 9 - master privileged
    #[inline(always)]
    pub fn mpriv(&mut self) -> MPRIV_W<'_, RIMC_ATTR11rs> {
        MPRIV_W::new(self, 9)
    }
}
/**RIFSC RIMC master attribute register 11

You can [`read`](crate::Reg::read) this register and get [`rimc_attr11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rimc_attr11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RIFSC:RIMC_ATTR11)*/
pub struct RIMC_ATTR11rs;
impl crate::RegisterSpec for RIMC_ATTR11rs {
    type Ux = u32;
}
///`read()` method returns [`rimc_attr11::R`](R) reader structure
impl crate::Readable for RIMC_ATTR11rs {}
///`write(|w| ..)` method takes [`rimc_attr11::W`](W) writer structure
impl crate::Writable for RIMC_ATTR11rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RIMC_ATTR11 to value 0
impl crate::Resettable for RIMC_ATTR11rs {}
