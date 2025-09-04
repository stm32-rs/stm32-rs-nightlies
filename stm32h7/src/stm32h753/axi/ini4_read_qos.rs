///Register `INI4_READ_QOS` reader
pub type R = crate::R<INI4_READ_QOSrs>;
///Register `INI4_READ_QOS` writer
pub type W = crate::W<INI4_READ_QOSrs>;
///Field `AR_QOS` reader - Read channel QoS setting
pub type AR_QOS_R = crate::FieldReader;
///Field `AR_QOS` writer - Read channel QoS setting
pub type AR_QOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    ///Bits 0:3 - Read channel QoS setting
    #[inline(always)]
    pub fn ar_qos(&self) -> AR_QOS_R {
        AR_QOS_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INI4_READ_QOS")
            .field("ar_qos", &self.ar_qos())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Read channel QoS setting
    #[inline(always)]
    pub fn ar_qos(&mut self) -> AR_QOS_W<INI4_READ_QOSrs> {
        AR_QOS_W::new(self, 0)
    }
}
/**AXI interconnect - INI x read QoS register

You can [`read`](crate::Reg::read) this register and get [`ini4_read_qos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ini4_read_qos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#AXI:INI4_READ_QOS)*/
pub struct INI4_READ_QOSrs;
impl crate::RegisterSpec for INI4_READ_QOSrs {
    type Ux = u32;
}
///`read()` method returns [`ini4_read_qos::R`](R) reader structure
impl crate::Readable for INI4_READ_QOSrs {}
///`write(|w| ..)` method takes [`ini4_read_qos::W`](W) writer structure
impl crate::Writable for INI4_READ_QOSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INI4_READ_QOS to value 0x04
impl crate::Resettable for INI4_READ_QOSrs {
    const RESET_VALUE: u32 = 0x04;
}
