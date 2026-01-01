///Register `IPC4R2` reader
pub type R = crate::R<IPC4R2rs>;
///Register `IPC4R2` writer
pub type W = crate::W<IPC4R2rs>;
///Field `SVCMAPPING` reader - Non-user, must be kept at reset value.
pub type SVCMAPPING_R = crate::FieldReader;
///Field `SVCMAPPING` writer - Non-user, must be kept at reset value.
pub type SVCMAPPING_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `WLRU` reader - Ratio for WLRU\[3:0\] arbitration
pub type WLRU_R = crate::FieldReader;
///Field `WLRU` writer - Ratio for WLRU\[3:0\] arbitration
pub type WLRU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 8:11 - Non-user, must be kept at reset value.
    #[inline(always)]
    pub fn svcmapping(&self) -> SVCMAPPING_R {
        SVCMAPPING_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:19 - Ratio for WLRU\[3:0\] arbitration
    #[inline(always)]
    pub fn wlru(&self) -> WLRU_R {
        WLRU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPC4R2")
            .field("svcmapping", &self.svcmapping())
            .field("wlru", &self.wlru())
            .finish()
    }
}
impl W {
    ///Bits 8:11 - Non-user, must be kept at reset value.
    #[inline(always)]
    pub fn svcmapping(&mut self) -> SVCMAPPING_W<'_, IPC4R2rs> {
        SVCMAPPING_W::new(self, 8)
    }
    ///Bits 16:19 - Ratio for WLRU\[3:0\] arbitration
    #[inline(always)]
    pub fn wlru(&mut self) -> WLRU_W<'_, IPC4R2rs> {
        WLRU_W::new(self, 16)
    }
}
/**DCMIPP IPPLUG Clientx register 2

You can [`read`](crate::Reg::read) this register and get [`ipc4r2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc4r2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMIPP:IPC4R2)*/
pub struct IPC4R2rs;
impl crate::RegisterSpec for IPC4R2rs {
    type Ux = u32;
}
///`read()` method returns [`ipc4r2::R`](R) reader structure
impl crate::Readable for IPC4R2rs {}
///`write(|w| ..)` method takes [`ipc4r2::W`](W) writer structure
impl crate::Writable for IPC4R2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IPC4R2 to value 0x0001_0000
impl crate::Resettable for IPC4R2rs {
    const RESET_VALUE: u32 = 0x0001_0000;
}
