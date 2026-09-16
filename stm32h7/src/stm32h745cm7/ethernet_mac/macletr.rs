///Register `MACLETR` reader
pub type R = crate::R<MACLETRrs>;
///Register `MACLETR` writer
pub type W = crate::W<MACLETRrs>;
///Field `LPIET` reader - LPI Entry Timer
pub type LPIET_R = crate::FieldReader<u32>;
///Field `LPIET` writer - LPI Entry Timer
pub type LPIET_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    ///Bits 3:19 - LPI Entry Timer
    #[inline(always)]
    pub fn lpiet(&self) -> LPIET_R {
        LPIET_R::new((self.bits >> 3) & 0x0001_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACLETR")
            .field("lpiet", &self.lpiet())
            .finish()
    }
}
impl W {
    ///Bits 3:19 - LPI Entry Timer
    #[inline(always)]
    pub fn lpiet(&mut self) -> LPIET_W<'_, MACLETRrs> {
        LPIET_W::new(self, 3)
    }
}
/**LPI entry timer register

You can [`read`](crate::Reg::read) this register and get [`macletr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macletr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM7.html#Ethernet_MAC:MACLETR)*/
pub struct MACLETRrs;
impl crate::RegisterSpec for MACLETRrs {
    type Ux = u32;
}
///`read()` method returns [`macletr::R`](R) reader structure
impl crate::Readable for MACLETRrs {}
///`write(|w| ..)` method takes [`macletr::W`](W) writer structure
impl crate::Writable for MACLETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACLETR to value 0
impl crate::Resettable for MACLETRrs {}
