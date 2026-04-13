///Register `INI6_WRITE_QOS` reader
pub type R = crate::R<INI6_WRITE_QOSrs>;
///Register `INI6_WRITE_QOS` writer
pub type W = crate::W<INI6_WRITE_QOSrs>;
///Field `AW_QOS` reader - Write channel QoS setting
pub type AW_QOS_R = crate::FieldReader;
///Field `AW_QOS` writer - Write channel QoS setting
pub type AW_QOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Write channel QoS setting
    #[inline(always)]
    pub fn aw_qos(&self) -> AW_QOS_R {
        AW_QOS_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INI6_WRITE_QOS")
            .field("aw_qos", &self.aw_qos())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Write channel QoS setting
    #[inline(always)]
    pub fn aw_qos(&mut self) -> AW_QOS_W<'_, INI6_WRITE_QOSrs> {
        AW_QOS_W::new(self, 0)
    }
}
/**AXI interconnect - INI x write QoS register

You can [`read`](crate::Reg::read) this register and get [`ini6_write_qos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ini6_write_qos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM7.html#AXI:INI6_WRITE_QOS)*/
pub struct INI6_WRITE_QOSrs;
impl crate::RegisterSpec for INI6_WRITE_QOSrs {
    type Ux = u32;
}
///`read()` method returns [`ini6_write_qos::R`](R) reader structure
impl crate::Readable for INI6_WRITE_QOSrs {}
///`write(|w| ..)` method takes [`ini6_write_qos::W`](W) writer structure
impl crate::Writable for INI6_WRITE_QOSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INI6_WRITE_QOS to value 0x04
impl crate::Resettable for INI6_WRITE_QOSrs {
    const RESET_VALUE: u32 = 0x04;
}
