///Register `IPGR2` reader
pub type R = crate::R<IPGR2rs>;
///Register `IPGR2` writer
pub type W = crate::W<IPGR2rs>;
///Field `PSTART` reader - Request to lock the IP-Plug, to allow reconfiguration.
pub type PSTART_R = crate::BitReader;
///Field `PSTART` writer - Request to lock the IP-Plug, to allow reconfiguration.
pub type PSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Request to lock the IP-Plug, to allow reconfiguration.
    #[inline(always)]
    pub fn pstart(&self) -> PSTART_R {
        PSTART_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPGR2")
            .field("pstart", &self.pstart())
            .finish()
    }
}
impl W {
    ///Bit 0 - Request to lock the IP-Plug, to allow reconfiguration.
    #[inline(always)]
    pub fn pstart(&mut self) -> PSTART_W<'_, IPGR2rs> {
        PSTART_W::new(self, 0)
    }
}
/**DCMIPP IPPLUG global register 2

You can [`read`](crate::Reg::read) this register and get [`ipgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMIPP:IPGR2)*/
pub struct IPGR2rs;
impl crate::RegisterSpec for IPGR2rs {
    type Ux = u32;
}
///`read()` method returns [`ipgr2::R`](R) reader structure
impl crate::Readable for IPGR2rs {}
///`write(|w| ..)` method takes [`ipgr2::W`](W) writer structure
impl crate::Writable for IPGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IPGR2 to value 0
impl crate::Resettable for IPGR2rs {}
