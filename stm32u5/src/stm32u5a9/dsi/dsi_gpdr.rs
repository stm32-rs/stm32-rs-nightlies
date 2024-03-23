#[doc = "Register `DSI_GPDR` reader"]
pub type R = crate::R<DSI_GPDRrs>;
#[doc = "Register `DSI_GPDR` writer"]
pub type W = crate::W<DSI_GPDRrs>;
#[doc = "Field `DATA1` reader - Payload byte 1 This field indicates the byte 1 of the packet payload."]
pub type DATA1_R = crate::FieldReader;
#[doc = "Field `DATA1` writer - Payload byte 1 This field indicates the byte 1 of the packet payload."]
pub type DATA1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA2` reader - Payload byte 2 This field indicates the byte 2 of the packet payload."]
pub type DATA2_R = crate::FieldReader;
#[doc = "Field `DATA2` writer - Payload byte 2 This field indicates the byte 2 of the packet payload."]
pub type DATA2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA3` reader - Payload byte 3 This field indicates the byte 3 of the packet payload."]
pub type DATA3_R = crate::FieldReader;
#[doc = "Field `DATA3` writer - Payload byte 3 This field indicates the byte 3 of the packet payload."]
pub type DATA3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA4` reader - Payload byte 4 This field indicates the byte 4 of the packet payload."]
pub type DATA4_R = crate::FieldReader;
#[doc = "Field `DATA4` writer - Payload byte 4 This field indicates the byte 4 of the packet payload."]
pub type DATA4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Payload byte 1 This field indicates the byte 1 of the packet payload."]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Payload byte 2 This field indicates the byte 2 of the packet payload."]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Payload byte 3 This field indicates the byte 3 of the packet payload."]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Payload byte 4 This field indicates the byte 4 of the packet payload."]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Payload byte 1 This field indicates the byte 1 of the packet payload."]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> DATA1_W<DSI_GPDRrs> {
        DATA1_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Payload byte 2 This field indicates the byte 2 of the packet payload."]
    #[inline(always)]
    #[must_use]
    pub fn data2(&mut self) -> DATA2_W<DSI_GPDRrs> {
        DATA2_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Payload byte 3 This field indicates the byte 3 of the packet payload."]
    #[inline(always)]
    #[must_use]
    pub fn data3(&mut self) -> DATA3_W<DSI_GPDRrs> {
        DATA3_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Payload byte 4 This field indicates the byte 4 of the packet payload."]
    #[inline(always)]
    #[must_use]
    pub fn data4(&mut self) -> DATA4_W<DSI_GPDRrs> {
        DATA4_W::new(self, 24)
    }
}
#[doc = "DSI Host generic payload data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_gpdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_gpdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_GPDRrs;
impl crate::RegisterSpec for DSI_GPDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_gpdr::R`](R) reader structure"]
impl crate::Readable for DSI_GPDRrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_gpdr::W`](W) writer structure"]
impl crate::Writable for DSI_GPDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_GPDR to value 0"]
impl crate::Resettable for DSI_GPDRrs {
    const RESET_VALUE: u32 = 0;
}
