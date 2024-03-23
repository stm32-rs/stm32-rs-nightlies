#[doc = "Register `BGPFCCR` reader"]
pub type R = crate::R<BGPFCCRrs>;
#[doc = "Register `BGPFCCR` writer"]
pub type W = crate::W<BGPFCCRrs>;
#[doc = "Field `CM` reader - Color mode These bits define the color format of the foreground image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
pub type CM_R = crate::FieldReader;
#[doc = "Field `CM` writer - Color mode These bits define the color format of the foreground image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
pub type CM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CCM` reader - CLUT Color mode These bits define the color format of the CLUT. This register can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only."]
pub type CCM_R = crate::BitReader;
#[doc = "Field `CCM` writer - CLUT Color mode These bits define the color format of the CLUT. This register can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only."]
pub type CCM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - Start This bit is set to start the automatic loading of the CLUT. This bit is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in the DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already on going (data transfer or automatic BackGround CLUT transfer)."]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Start This bit is set to start the automatic loading of the CLUT. This bit is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in the DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already on going (data transfer or automatic BackGround CLUT transfer)."]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS` reader - CLUT size These bits define the size of the CLUT used for the BG. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS\\[7:0\\]
+ 1."]
pub type CS_R = crate::FieldReader;
#[doc = "Field `CS` writer - CLUT size These bits define the size of the CLUT used for the BG. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS\\[7:0\\]
+ 1."]
pub type CS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AM` reader - Alpha mode These bits define which alpha channel value to be used for the background image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
pub type AM_R = crate::FieldReader;
#[doc = "Field `AM` writer - Alpha mode These bits define which alpha channel value to be used for the background image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
pub type AM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AI` reader - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
pub type AI_R = crate::BitReader;
#[doc = "Field `AI` writer - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
pub type AI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBS` reader - Red Blue Swap This bit allows to swap the R &amp;amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
pub type RBS_R = crate::BitReader;
#[doc = "Field `RBS` writer - Red Blue Swap This bit allows to swap the R &amp;amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
pub type RBS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALPHA` reader - Alpha value These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied with the original alpha value according to the alpha mode selected with bits AM\\[1: 0\\]. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type ALPHA_R = crate::FieldReader;
#[doc = "Field `ALPHA` writer - Alpha value These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied with the original alpha value according to the alpha mode selected with bits AM\\[1: 0\\]. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type ALPHA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Color mode These bits define the color format of the foreground image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - CLUT Color mode These bits define the color format of the CLUT. This register can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only."]
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Start This bit is set to start the automatic loading of the CLUT. This bit is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in the DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already on going (data transfer or automatic BackGround CLUT transfer)."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:15 - CLUT size These bits define the size of the CLUT used for the BG. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS\\[7:0\\]
+ 1."]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Alpha mode These bits define which alpha channel value to be used for the background image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Red Blue Swap This bit allows to swap the R &amp;amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
    #[inline(always)]
    pub fn rbs(&self) -> RBS_R {
        RBS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Alpha value These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied with the original alpha value according to the alpha mode selected with bits AM\\[1: 0\\]. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Color mode These bits define the color format of the foreground image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<BGPFCCRrs> {
        CM_W::new(self, 0)
    }
    #[doc = "Bit 4 - CLUT Color mode These bits define the color format of the CLUT. This register can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only."]
    #[inline(always)]
    #[must_use]
    pub fn ccm(&mut self) -> CCM_W<BGPFCCRrs> {
        CCM_W::new(self, 4)
    }
    #[doc = "Bit 5 - Start This bit is set to start the automatic loading of the CLUT. This bit is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in the DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already on going (data transfer or automatic BackGround CLUT transfer)."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<BGPFCCRrs> {
        START_W::new(self, 5)
    }
    #[doc = "Bits 8:15 - CLUT size These bits define the size of the CLUT used for the BG. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS\\[7:0\\]
+ 1."]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CS_W<BGPFCCRrs> {
        CS_W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Alpha mode These bits define which alpha channel value to be used for the background image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
    #[inline(always)]
    #[must_use]
    pub fn am(&mut self) -> AM_W<BGPFCCRrs> {
        AM_W::new(self, 16)
    }
    #[doc = "Bit 20 - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
    #[inline(always)]
    #[must_use]
    pub fn ai(&mut self) -> AI_W<BGPFCCRrs> {
        AI_W::new(self, 20)
    }
    #[doc = "Bit 21 - Red Blue Swap This bit allows to swap the R &amp;amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
    #[inline(always)]
    #[must_use]
    pub fn rbs(&mut self) -> RBS_W<BGPFCCRrs> {
        RBS_W::new(self, 21)
    }
    #[doc = "Bits 24:31 - Alpha value These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied with the original alpha value according to the alpha mode selected with bits AM\\[1: 0\\]. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    #[must_use]
    pub fn alpha(&mut self) -> ALPHA_W<BGPFCCRrs> {
        ALPHA_W::new(self, 24)
    }
}
#[doc = "DMA2D background PFC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgpfccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgpfccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BGPFCCRrs;
impl crate::RegisterSpec for BGPFCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bgpfccr::R`](R) reader structure"]
impl crate::Readable for BGPFCCRrs {}
#[doc = "`write(|w| ..)` method takes [`bgpfccr::W`](W) writer structure"]
impl crate::Writable for BGPFCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BGPFCCR to value 0"]
impl crate::Resettable for BGPFCCRrs {
    const RESET_VALUE: u32 = 0;
}
