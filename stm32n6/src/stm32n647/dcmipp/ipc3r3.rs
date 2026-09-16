///Register `IPC3R3` reader
pub type R = crate::R<IPC3R3rs>;
///Register `IPC3R3` writer
pub type W = crate::W<IPC3R3rs>;
///Field `DPREGSTART` reader - Start word (AXI width = 64 bits) of the FIFO of Clientx.
pub type DPREGSTART_R = crate::FieldReader<u16>;
///Field `DPREGSTART` writer - Start word (AXI width = 64 bits) of the FIFO of Clientx.
pub type DPREGSTART_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `DPREGEND` reader - End word (AXI width = 64 bits) of the FIFO of Clientx.
pub type DPREGEND_R = crate::FieldReader<u16>;
///Field `DPREGEND` writer - End word (AXI width = 64 bits) of the FIFO of Clientx.
pub type DPREGEND_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Start word (AXI width = 64 bits) of the FIFO of Clientx.
    #[inline(always)]
    pub fn dpregstart(&self) -> DPREGSTART_R {
        DPREGSTART_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 16:25 - End word (AXI width = 64 bits) of the FIFO of Clientx.
    #[inline(always)]
    pub fn dpregend(&self) -> DPREGEND_R {
        DPREGEND_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPC3R3")
            .field("dpregstart", &self.dpregstart())
            .field("dpregend", &self.dpregend())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Start word (AXI width = 64 bits) of the FIFO of Clientx.
    #[inline(always)]
    pub fn dpregstart(&mut self) -> DPREGSTART_W<'_, IPC3R3rs> {
        DPREGSTART_W::new(self, 0)
    }
    ///Bits 16:25 - End word (AXI width = 64 bits) of the FIFO of Clientx.
    #[inline(always)]
    pub fn dpregend(&mut self) -> DPREGEND_W<'_, IPC3R3rs> {
        DPREGEND_W::new(self, 16)
    }
}
/**DCMIPP IPPLUG Clientx register 3

You can [`read`](crate::Reg::read) this register and get [`ipc3r3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc3r3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMIPP:IPC3R3)*/
pub struct IPC3R3rs;
impl crate::RegisterSpec for IPC3R3rs {
    type Ux = u32;
}
///`read()` method returns [`ipc3r3::R`](R) reader structure
impl crate::Readable for IPC3R3rs {}
///`write(|w| ..)` method takes [`ipc3r3::W`](W) writer structure
impl crate::Writable for IPC3R3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IPC3R3 to value 0x018f_0140
impl crate::Resettable for IPC3R3rs {
    const RESET_VALUE: u32 = 0x018f_0140;
}
