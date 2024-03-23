#[doc = "Register `TOCC` reader"]
pub type R = crate::R<TOCCrs>;
#[doc = "Register `TOCC` writer"]
pub type W = crate::W<TOCCrs>;
#[doc = "Field `ETOC` reader - Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type ETOC_R = crate::BitReader;
#[doc = "Field `ETOC` writer - Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type ETOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS` reader - Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\\[TOP\\]
and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\\[TOP\\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type TOS_R = crate::FieldReader;
#[doc = "Field `TOS` writer - Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\\[TOP\\]
and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\\[TOP\\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type TOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOP` reader - Timeout period Start value of the timeout counter (down-counter). Configures the timeout period."]
pub type TOP_R = crate::FieldReader<u16>;
#[doc = "Field `TOP` writer - Timeout period Start value of the timeout counter (down-counter). Configures the timeout period."]
pub type TOP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn etoc(&self) -> ETOC_R {
        ETOC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\\[TOP\\]
and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\\[TOP\\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn tos(&self) -> TOS_R {
        TOS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 16:31 - Timeout period Start value of the timeout counter (down-counter). Configures the timeout period."]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn etoc(&mut self) -> ETOC_W<TOCCrs> {
        ETOC_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\\[TOP\\]
and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\\[TOP\\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn tos(&mut self) -> TOS_W<TOCCrs> {
        TOS_W::new(self, 1)
    }
    #[doc = "Bits 16:31 - Timeout period Start value of the timeout counter (down-counter). Configures the timeout period."]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TOP_W<TOCCrs> {
        TOP_W::new(self, 16)
    }
}
#[doc = "FDCAN timeout counter configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOCCrs;
impl crate::RegisterSpec for TOCCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tocc::R`](R) reader structure"]
impl crate::Readable for TOCCrs {}
#[doc = "`write(|w| ..)` method takes [`tocc::W`](W) writer structure"]
impl crate::Writable for TOCCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOCC to value 0xffff_0000"]
impl crate::Resettable for TOCCrs {
    const RESET_VALUE: u32 = 0xffff_0000;
}
