///Register `OPFCCR` reader
pub type R = crate::R<OPFCCRrs>;
///Register `OPFCCR` writer
pub type W = crate::W<OPFCCRrs>;
/**Color mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CM {
    ///0: ARGB8888
    Argb8888 = 0,
    ///1: RGB888
    Rgb888 = 1,
    ///2: RGB565
    Rgb565 = 2,
    ///3: ARGB1555
    Argb1555 = 3,
    ///4: ARGB4444
    Argb4444 = 4,
}
impl From<CM> for u8 {
    #[inline(always)]
    fn from(variant: CM) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CM {
    type Ux = u8;
}
impl crate::IsEnum for CM {}
///Field `CM` reader - Color mode
pub type CM_R = crate::FieldReader<CM>;
impl CM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CM> {
        match self.bits {
            0 => Some(CM::Argb8888),
            1 => Some(CM::Rgb888),
            2 => Some(CM::Rgb565),
            3 => Some(CM::Argb1555),
            4 => Some(CM::Argb4444),
            _ => None,
        }
    }
    ///ARGB8888
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == CM::Argb8888
    }
    ///RGB888
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == CM::Rgb888
    }
    ///RGB565
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        *self == CM::Rgb565
    }
    ///ARGB1555
    #[inline(always)]
    pub fn is_argb1555(&self) -> bool {
        *self == CM::Argb1555
    }
    ///ARGB4444
    #[inline(always)]
    pub fn is_argb4444(&self) -> bool {
        *self == CM::Argb4444
    }
}
///Field `CM` writer - Color mode
pub type CM_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CM>;
impl<'a, REG> CM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///ARGB8888
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Argb8888)
    }
    ///RGB888
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Rgb888)
    }
    ///RGB565
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Rgb565)
    }
    ///ARGB1555
    #[inline(always)]
    pub fn argb1555(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Argb1555)
    }
    ///ARGB4444
    #[inline(always)]
    pub fn argb4444(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Argb4444)
    }
}
impl R {
    ///Bits 0:2 - Color mode
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPFCCR").field("cm", &self.cm()).finish()
    }
}
impl W {
    ///Bits 0:2 - Color mode
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W<'_, OPFCCRrs> {
        CM_W::new(self, 0)
    }
}
/**output PFC control register

You can [`read`](crate::Reg::read) this register and get [`opfccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opfccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F750.html#DMA2D:OPFCCR)*/
pub struct OPFCCRrs;
impl crate::RegisterSpec for OPFCCRrs {
    type Ux = u32;
}
///`read()` method returns [`opfccr::R`](R) reader structure
impl crate::Readable for OPFCCRrs {}
///`write(|w| ..)` method takes [`opfccr::W`](W) writer structure
impl crate::Writable for OPFCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPFCCR to value 0
impl crate::Resettable for OPFCCRrs {}
