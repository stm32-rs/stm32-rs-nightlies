#[doc = "Register `DSI_TCCR1` reader"]
pub type R = crate::R<DSI_TCCR1rs>;
#[doc = "Register `DSI_TCCR1` writer"]
pub type W = crate::W<DSI_TCCR1rs>;
#[doc = "Field `HSRD_TOCNT` reader - High-speed read timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a high-speed read operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts."]
pub type HSRD_TOCNT_R = crate::FieldReader<u16>;
#[doc = "Field `HSRD_TOCNT` writer - High-speed read timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a high-speed read operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts."]
pub type HSRD_TOCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - High-speed read timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a high-speed read operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts."]
    #[inline(always)]
    pub fn hsrd_tocnt(&self) -> HSRD_TOCNT_R {
        HSRD_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - High-speed read timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a high-speed read operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn hsrd_tocnt(&mut self) -> HSRD_TOCNT_W<DSI_TCCR1rs> {
        HSRD_TOCNT_W::new(self, 0)
    }
}
#[doc = "DSI Host timeout counter configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_tccr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_tccr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_TCCR1rs;
impl crate::RegisterSpec for DSI_TCCR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_tccr1::R`](R) reader structure"]
impl crate::Readable for DSI_TCCR1rs {}
#[doc = "`write(|w| ..)` method takes [`dsi_tccr1::W`](W) writer structure"]
impl crate::Writable for DSI_TCCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_TCCR1 to value 0"]
impl crate::Resettable for DSI_TCCR1rs {
    const RESET_VALUE: u32 = 0;
}
