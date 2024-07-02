///Register `ETH_MACLETR` reader
pub type R = crate::R<ETH_MACLETRrs>;
///Register `ETH_MACLETR` writer
pub type W = crate::W<ETH_MACLETRrs>;
///Field `LPIET` reader - LPIET
pub type LPIET_R = crate::FieldReader<u32>;
///Field `LPIET` writer - LPIET
pub type LPIET_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    ///Bits 3:19 - LPIET
    #[inline(always)]
    pub fn lpiet(&self) -> LPIET_R {
        LPIET_R::new((self.bits >> 3) & 0x0001_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_MACLETR")
            .field("lpiet", &self.lpiet())
            .finish()
    }
}
impl W {
    ///Bits 3:19 - LPIET
    #[inline(always)]
    #[must_use]
    pub fn lpiet(&mut self) -> LPIET_W<ETH_MACLETRrs> {
        LPIET_W::new(self, 3)
    }
}
/**The LPI Entry Timer Register is used to store the LPI Idle Timer Value in Micro-Seconds.

You can [`read`](crate::Reg::read) this register and get [`eth_macletr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_macletr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:ETH_MACLETR)*/
pub struct ETH_MACLETRrs;
impl crate::RegisterSpec for ETH_MACLETRrs {
    type Ux = u32;
}
///`read()` method returns [`eth_macletr::R`](R) reader structure
impl crate::Readable for ETH_MACLETRrs {}
///`write(|w| ..)` method takes [`eth_macletr::W`](W) writer structure
impl crate::Writable for ETH_MACLETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETH_MACLETR to value 0
impl crate::Resettable for ETH_MACLETRrs {
    const RESET_VALUE: u32 = 0;
}
