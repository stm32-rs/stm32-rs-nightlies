///Register `BGPFCCR` reader
pub type R = crate::R<BGPFCCRrs>;
///Register `BGPFCCR` writer
pub type W = crate::W<BGPFCCRrs>;
///Field `CM` reader - Color mode These bits define the color format of the foreground image. Others: Reserved
pub type CM_R = crate::FieldReader;
///Field `CM` writer - Color mode These bits define the color format of the foreground image. Others: Reserved
pub type CM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CCM` reader - CLUT color mode These bits define the color format of the CLUT.
pub type CCM_R = crate::BitReader;
///Field `CCM` writer - CLUT color mode These bits define the color format of the CLUT.
pub type CCM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `START` reader - Start This bit is set to start the automatic loading of the CLUT. This bit is automatically reset: at the end of the transfer when the transfer is aborted by the user by setting ABORT bit in DMA2D_CR when a transfer error occurs when the transfer has not started due to a configuration error or another transfer operation already on going (data transfer or automatic background CLUT transfer)
pub type START_R = crate::BitReader;
///Field `START` writer - Start This bit is set to start the automatic loading of the CLUT. This bit is automatically reset: at the end of the transfer when the transfer is aborted by the user by setting ABORT bit in DMA2D_CR when a transfer error occurs when the transfer has not started due to a configuration error or another transfer operation already on going (data transfer or automatic background CLUT transfer)
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CS` reader - CLUT size These bits define the size of the CLUT used for the BG. The number of CLUT entries is equal to CS\[7:0\] + 1.
pub type CS_R = crate::FieldReader;
///Field `CS` writer - CLUT size These bits define the size of the CLUT used for the BG. The number of CLUT entries is equal to CS\[7:0\] + 1.
pub type CS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `AM` reader - Alpha mode These bits define which alpha channel value to be used for the background image. Others: Reserved
pub type AM_R = crate::FieldReader;
///Field `AM` writer - Alpha mode These bits define which alpha channel value to be used for the background image. Others: Reserved
pub type AM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `AI` reader - Alpha Inverted This bit inverts the alpha value.
pub type AI_R = crate::BitReader;
///Field `AI` writer - Alpha Inverted This bit inverts the alpha value.
pub type AI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RBS` reader - Red/Blue swap This bit allows to swap Red and Blue to support BGR or ABGR color formats.
pub type RBS_R = crate::BitReader;
///Field `RBS` writer - Red/Blue swap This bit allows to swap Red and Blue to support BGR or ABGR color formats.
pub type RBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALPHA` reader - Alpha value These bits define a fixed alpha channel value which can replace the original alpha value, or be multiplied with the original alpha value according to the alpha mode selected with AM\[1:0\].
pub type ALPHA_R = crate::FieldReader;
///Field `ALPHA` writer - Alpha value These bits define a fixed alpha channel value which can replace the original alpha value, or be multiplied with the original alpha value according to the alpha mode selected with AM\[1:0\].
pub type ALPHA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:3 - Color mode These bits define the color format of the foreground image. Others: Reserved
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - CLUT color mode These bits define the color format of the CLUT.
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Start This bit is set to start the automatic loading of the CLUT. This bit is automatically reset: at the end of the transfer when the transfer is aborted by the user by setting ABORT bit in DMA2D_CR when a transfer error occurs when the transfer has not started due to a configuration error or another transfer operation already on going (data transfer or automatic background CLUT transfer)
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:15 - CLUT size These bits define the size of the CLUT used for the BG. The number of CLUT entries is equal to CS\[7:0\] + 1.
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:17 - Alpha mode These bits define which alpha channel value to be used for the background image. Others: Reserved
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 20 - Alpha Inverted This bit inverts the alpha value.
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Red/Blue swap This bit allows to swap Red and Blue to support BGR or ABGR color formats.
    #[inline(always)]
    pub fn rbs(&self) -> RBS_R {
        RBS_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 24:31 - Alpha value These bits define a fixed alpha channel value which can replace the original alpha value, or be multiplied with the original alpha value according to the alpha mode selected with AM\[1:0\].
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BGPFCCR")
            .field("cm", &self.cm())
            .field("ccm", &self.ccm())
            .field("start", &self.start())
            .field("cs", &self.cs())
            .field("am", &self.am())
            .field("ai", &self.ai())
            .field("rbs", &self.rbs())
            .field("alpha", &self.alpha())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Color mode These bits define the color format of the foreground image. Others: Reserved
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W<'_, BGPFCCRrs> {
        CM_W::new(self, 0)
    }
    ///Bit 4 - CLUT color mode These bits define the color format of the CLUT.
    #[inline(always)]
    pub fn ccm(&mut self) -> CCM_W<'_, BGPFCCRrs> {
        CCM_W::new(self, 4)
    }
    ///Bit 5 - Start This bit is set to start the automatic loading of the CLUT. This bit is automatically reset: at the end of the transfer when the transfer is aborted by the user by setting ABORT bit in DMA2D_CR when a transfer error occurs when the transfer has not started due to a configuration error or another transfer operation already on going (data transfer or automatic background CLUT transfer)
    #[inline(always)]
    pub fn start(&mut self) -> START_W<'_, BGPFCCRrs> {
        START_W::new(self, 5)
    }
    ///Bits 8:15 - CLUT size These bits define the size of the CLUT used for the BG. The number of CLUT entries is equal to CS\[7:0\] + 1.
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W<'_, BGPFCCRrs> {
        CS_W::new(self, 8)
    }
    ///Bits 16:17 - Alpha mode These bits define which alpha channel value to be used for the background image. Others: Reserved
    #[inline(always)]
    pub fn am(&mut self) -> AM_W<'_, BGPFCCRrs> {
        AM_W::new(self, 16)
    }
    ///Bit 20 - Alpha Inverted This bit inverts the alpha value.
    #[inline(always)]
    pub fn ai(&mut self) -> AI_W<'_, BGPFCCRrs> {
        AI_W::new(self, 20)
    }
    ///Bit 21 - Red/Blue swap This bit allows to swap Red and Blue to support BGR or ABGR color formats.
    #[inline(always)]
    pub fn rbs(&mut self) -> RBS_W<'_, BGPFCCRrs> {
        RBS_W::new(self, 21)
    }
    ///Bits 24:31 - Alpha value These bits define a fixed alpha channel value which can replace the original alpha value, or be multiplied with the original alpha value according to the alpha mode selected with AM\[1:0\].
    #[inline(always)]
    pub fn alpha(&mut self) -> ALPHA_W<'_, BGPFCCRrs> {
        ALPHA_W::new(self, 24)
    }
}
/**DMA2D background PFC control register

You can [`read`](crate::Reg::read) this register and get [`bgpfccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgpfccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DMA2D:BGPFCCR)*/
pub struct BGPFCCRrs;
impl crate::RegisterSpec for BGPFCCRrs {
    type Ux = u32;
}
///`read()` method returns [`bgpfccr::R`](R) reader structure
impl crate::Readable for BGPFCCRrs {}
///`write(|w| ..)` method takes [`bgpfccr::W`](W) writer structure
impl crate::Writable for BGPFCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BGPFCCR to value 0
impl crate::Resettable for BGPFCCRrs {}
