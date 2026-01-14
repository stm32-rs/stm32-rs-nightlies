///Register `IPC5R1` reader
pub type R = crate::R<IPC5R1rs>;
///Register `IPC5R1` writer
pub type W = crate::W<IPC5R1rs>;
///Field `TRAFFIC` reader - Burst size as power of 2 of 8-byte units
pub type TRAFFIC_R = crate::FieldReader;
///Field `TRAFFIC` writer - Burst size as power of 2 of 8-byte units
pub type TRAFFIC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OTR` reader - Maximum outstanding transactions
pub type OTR_R = crate::FieldReader;
///Field `OTR` writer - Maximum outstanding transactions
pub type OTR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:2 - Burst size as power of 2 of 8-byte units
    #[inline(always)]
    pub fn traffic(&self) -> TRAFFIC_R {
        TRAFFIC_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:11 - Maximum outstanding transactions
    #[inline(always)]
    pub fn otr(&self) -> OTR_R {
        OTR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPC5R1")
            .field("traffic", &self.traffic())
            .field("otr", &self.otr())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Burst size as power of 2 of 8-byte units
    #[inline(always)]
    pub fn traffic(&mut self) -> TRAFFIC_W<'_, IPC5R1rs> {
        TRAFFIC_W::new(self, 0)
    }
    ///Bits 8:11 - Maximum outstanding transactions
    #[inline(always)]
    pub fn otr(&mut self) -> OTR_W<'_, IPC5R1rs> {
        OTR_W::new(self, 8)
    }
}
/**DCMIPP IPPLUG Clientx register 1

You can [`read`](crate::Reg::read) this register and get [`ipc5r1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc5r1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMIPP:IPC5R1)*/
pub struct IPC5R1rs;
impl crate::RegisterSpec for IPC5R1rs {
    type Ux = u32;
}
///`read()` method returns [`ipc5r1::R`](R) reader structure
impl crate::Readable for IPC5R1rs {}
///`write(|w| ..)` method takes [`ipc5r1::W`](W) writer structure
impl crate::Writable for IPC5R1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IPC5R1 to value 0x04
impl crate::Resettable for IPC5R1rs {
    const RESET_VALUE: u32 = 0x04;
}
