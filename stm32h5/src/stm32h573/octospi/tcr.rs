#[doc = "Register `TCR` reader"]
pub type R = crate::R<TCRrs>;
#[doc = "Register `TCR` writer"]
pub type W = crate::W<TCRrs>;
#[doc = "Field `DCYC` reader - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least six dummy cycles when using memories with DQS activated."]
pub type DCYC_R = crate::FieldReader;
#[doc = "Field `DCYC` writer - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least six dummy cycles when using memories with DQS activated."]
pub type DCYC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DHQC` reader - Delay hold quarter cycle"]
pub type DHQC_R = crate::BitReader;
#[doc = "Field `DHQC` writer - Delay hold quarter cycle"]
pub type DHQC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSHIFT` reader - Sample shift By default, the OCTOSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The software must ensure that SSHIFT = 0 when the data phase is configured in DTR mode (when DDTR = 1.)"]
pub type SSHIFT_R = crate::BitReader;
#[doc = "Field `SSHIFT` writer - Sample shift By default, the OCTOSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The software must ensure that SSHIFT = 0 when the data phase is configured in DTR mode (when DDTR = 1.)"]
pub type SSHIFT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least six dummy cycles when using memories with DQS activated."]
    #[inline(always)]
    pub fn dcyc(&self) -> DCYC_R {
        DCYC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 28 - Delay hold quarter cycle"]
    #[inline(always)]
    pub fn dhqc(&self) -> DHQC_R {
        DHQC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Sample shift By default, the OCTOSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The software must ensure that SSHIFT = 0 when the data phase is configured in DTR mode (when DDTR = 1.)"]
    #[inline(always)]
    pub fn sshift(&self) -> SSHIFT_R {
        SSHIFT_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least six dummy cycles when using memories with DQS activated."]
    #[inline(always)]
    #[must_use]
    pub fn dcyc(&mut self) -> DCYC_W<TCRrs> {
        DCYC_W::new(self, 0)
    }
    #[doc = "Bit 28 - Delay hold quarter cycle"]
    #[inline(always)]
    #[must_use]
    pub fn dhqc(&mut self) -> DHQC_W<TCRrs> {
        DHQC_W::new(self, 28)
    }
    #[doc = "Bit 30 - Sample shift By default, the OCTOSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The software must ensure that SSHIFT = 0 when the data phase is configured in DTR mode (when DDTR = 1.)"]
    #[inline(always)]
    #[must_use]
    pub fn sshift(&mut self) -> SSHIFT_W<TCRrs> {
        SSHIFT_W::new(self, 30)
    }
}
#[doc = "OCTOSPI timing configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCRrs;
impl crate::RegisterSpec for TCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcr::R`](R) reader structure"]
impl crate::Readable for TCRrs {}
#[doc = "`write(|w| ..)` method takes [`tcr::W`](W) writer structure"]
impl crate::Writable for TCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TCRrs {
    const RESET_VALUE: u32 = 0;
}
