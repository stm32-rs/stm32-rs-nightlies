///Register `M3_WRITE_QOS` reader
pub type R = crate::R<M3_WRITE_QOSrs>;
///Register `M3_WRITE_QOS` writer
pub type W = crate::W<M3_WRITE_QOSrs>;
///Field `AW_QOS` reader - AW_QOS
pub type AW_QOS_R = crate::FieldReader;
///Field `AW_QOS` writer - AW_QOS
pub type AW_QOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - AW_QOS
    #[inline(always)]
    pub fn aw_qos(&self) -> AW_QOS_R {
        AW_QOS_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M3_WRITE_QOS")
            .field("aw_qos", &self.aw_qos())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - AW_QOS
    #[inline(always)]
    pub fn aw_qos(&mut self) -> AW_QOS_W<M3_WRITE_QOSrs> {
        AW_QOS_W::new(self, 0)
    }
}
/**AXIMC master 3 write priority register

You can [`read`](crate::Reg::read) this register and get [`m3_write_qos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3_write_qos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#AXIMC_Mx:M3_WRITE_QOS)*/
pub struct M3_WRITE_QOSrs;
impl crate::RegisterSpec for M3_WRITE_QOSrs {
    type Ux = u32;
}
///`read()` method returns [`m3_write_qos::R`](R) reader structure
impl crate::Readable for M3_WRITE_QOSrs {}
///`write(|w| ..)` method takes [`m3_write_qos::W`](W) writer structure
impl crate::Writable for M3_WRITE_QOSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M3_WRITE_QOS to value 0x07
impl crate::Resettable for M3_WRITE_QOSrs {
    const RESET_VALUE: u32 = 0x07;
}
