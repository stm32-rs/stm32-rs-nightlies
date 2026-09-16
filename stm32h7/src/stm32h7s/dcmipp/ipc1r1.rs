///Register `IPC1R1` reader
pub type R = crate::R<IPC1R1rs>;
///Register `IPC1R1` writer
pub type W = crate::W<IPC1R1rs>;
///Field `TRAFFIC` reader - Burst size as power of 2 of 8-byte units Other values: Reserved
pub type TRAFFIC_R = crate::FieldReader;
///Field `TRAFFIC` writer - Burst size as power of 2 of 8-byte units Other values: Reserved
pub type TRAFFIC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OTR` reader - Maximum outstanding transactions ... Other values are not allowed.
pub type OTR_R = crate::FieldReader;
///Field `OTR` writer - Maximum outstanding transactions ... Other values are not allowed.
pub type OTR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - Burst size as power of 2 of 8-byte units Other values: Reserved
    #[inline(always)]
    pub fn traffic(&self) -> TRAFFIC_R {
        TRAFFIC_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:9 - Maximum outstanding transactions ... Other values are not allowed.
    #[inline(always)]
    pub fn otr(&self) -> OTR_R {
        OTR_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPC1R1")
            .field("traffic", &self.traffic())
            .field("otr", &self.otr())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Burst size as power of 2 of 8-byte units Other values: Reserved
    #[inline(always)]
    pub fn traffic(&mut self) -> TRAFFIC_W<'_, IPC1R1rs> {
        TRAFFIC_W::new(self, 0)
    }
    ///Bits 8:9 - Maximum outstanding transactions ... Other values are not allowed.
    #[inline(always)]
    pub fn otr(&mut self) -> OTR_W<'_, IPC1R1rs> {
        OTR_W::new(self, 8)
    }
}
/**DCMIPP IP-Plug Clientx register 1

You can [`read`](crate::Reg::read) this register and get [`ipc1r1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc1r1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DCMIPP:IPC1R1)*/
pub struct IPC1R1rs;
impl crate::RegisterSpec for IPC1R1rs {
    type Ux = u32;
}
///`read()` method returns [`ipc1r1::R`](R) reader structure
impl crate::Readable for IPC1R1rs {}
///`write(|w| ..)` method takes [`ipc1r1::W`](W) writer structure
impl crate::Writable for IPC1R1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IPC1R1 to value 0x03
impl crate::Resettable for IPC1R1rs {
    const RESET_VALUE: u32 = 0x03;
}
