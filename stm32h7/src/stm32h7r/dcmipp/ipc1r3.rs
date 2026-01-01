///Register `IPC1R3` reader
pub type R = crate::R<IPC1R3rs>;
///Register `IPC1R3` writer
pub type W = crate::W<IPC1R3rs>;
///Field `DPREGSTART` reader - Start word (AXI width = 64 bits) of the FIFO of Clientx.
pub type DPREGSTART_R = crate::FieldReader;
///Field `DPREGSTART` writer - Start word (AXI width = 64 bits) of the FIFO of Clientx.
pub type DPREGSTART_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DPREGEND` reader - End word (AXI width = 64 bits) of the FIFO of Clientx. The addressed word is included in the FIFO, so that next DPREGSTART is DPREGEND + 1.
pub type DPREGEND_R = crate::FieldReader;
///Field `DPREGEND` writer - End word (AXI width = 64 bits) of the FIFO of Clientx. The addressed word is included in the FIFO, so that next DPREGSTART is DPREGEND + 1.
pub type DPREGEND_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - Start word (AXI width = 64 bits) of the FIFO of Clientx.
    #[inline(always)]
    pub fn dpregstart(&self) -> DPREGSTART_R {
        DPREGSTART_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 16:20 - End word (AXI width = 64 bits) of the FIFO of Clientx. The addressed word is included in the FIFO, so that next DPREGSTART is DPREGEND + 1.
    #[inline(always)]
    pub fn dpregend(&self) -> DPREGEND_R {
        DPREGEND_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPC1R3")
            .field("dpregstart", &self.dpregstart())
            .field("dpregend", &self.dpregend())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Start word (AXI width = 64 bits) of the FIFO of Clientx.
    #[inline(always)]
    pub fn dpregstart(&mut self) -> DPREGSTART_W<'_, IPC1R3rs> {
        DPREGSTART_W::new(self, 0)
    }
    ///Bits 16:20 - End word (AXI width = 64 bits) of the FIFO of Clientx. The addressed word is included in the FIFO, so that next DPREGSTART is DPREGEND + 1.
    #[inline(always)]
    pub fn dpregend(&mut self) -> DPREGEND_W<'_, IPC1R3rs> {
        DPREGEND_W::new(self, 16)
    }
}
/**DCMIPP IP-Plug Clientx register 3

You can [`read`](crate::Reg::read) this register and get [`ipc1r3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc1r3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:IPC1R3)*/
pub struct IPC1R3rs;
impl crate::RegisterSpec for IPC1R3rs {
    type Ux = u32;
}
///`read()` method returns [`ipc1r3::R`](R) reader structure
impl crate::Readable for IPC1R3rs {}
///`write(|w| ..)` method takes [`ipc1r3::W`](W) writer structure
impl crate::Writable for IPC1R3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IPC1R3 to value 0x001f_0000
impl crate::Resettable for IPC1R3rs {
    const RESET_VALUE: u32 = 0x001f_0000;
}
