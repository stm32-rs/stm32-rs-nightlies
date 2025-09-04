///Register `BGPFCCR` reader
pub type R = crate::R<BGPFCCRrs>;
///Register `BGPFCCR` writer
pub type W = crate::W<BGPFCCRrs>;
/**Color mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CM {
    ///0: Color mode ARGB8888
    Argb8888 = 0,
    ///1: Color mode RGB888
    Rgb888 = 1,
    ///2: Color mode RGB565
    Rgb565 = 2,
    ///3: Color mode ARGB1555
    Argb1555 = 3,
    ///4: Color mode ARGB4444
    Argb4444 = 4,
    ///5: Color mode L8
    L8 = 5,
    ///6: Color mode AL44
    Al44 = 6,
    ///7: Color mode AL88
    Al88 = 7,
    ///8: Color mode L4
    L4 = 8,
    ///9: Color mode A8
    A8 = 9,
    ///10: Color mode A4
    A4 = 10,
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
            5 => Some(CM::L8),
            6 => Some(CM::Al44),
            7 => Some(CM::Al88),
            8 => Some(CM::L4),
            9 => Some(CM::A8),
            10 => Some(CM::A4),
            _ => None,
        }
    }
    ///Color mode ARGB8888
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == CM::Argb8888
    }
    ///Color mode RGB888
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == CM::Rgb888
    }
    ///Color mode RGB565
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        *self == CM::Rgb565
    }
    ///Color mode ARGB1555
    #[inline(always)]
    pub fn is_argb1555(&self) -> bool {
        *self == CM::Argb1555
    }
    ///Color mode ARGB4444
    #[inline(always)]
    pub fn is_argb4444(&self) -> bool {
        *self == CM::Argb4444
    }
    ///Color mode L8
    #[inline(always)]
    pub fn is_l8(&self) -> bool {
        *self == CM::L8
    }
    ///Color mode AL44
    #[inline(always)]
    pub fn is_al44(&self) -> bool {
        *self == CM::Al44
    }
    ///Color mode AL88
    #[inline(always)]
    pub fn is_al88(&self) -> bool {
        *self == CM::Al88
    }
    ///Color mode L4
    #[inline(always)]
    pub fn is_l4(&self) -> bool {
        *self == CM::L4
    }
    ///Color mode A8
    #[inline(always)]
    pub fn is_a8(&self) -> bool {
        *self == CM::A8
    }
    ///Color mode A4
    #[inline(always)]
    pub fn is_a4(&self) -> bool {
        *self == CM::A4
    }
}
///Field `CM` writer - Color mode
pub type CM_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CM>;
impl<'a, REG> CM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Color mode ARGB8888
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Argb8888)
    }
    ///Color mode RGB888
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Rgb888)
    }
    ///Color mode RGB565
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Rgb565)
    }
    ///Color mode ARGB1555
    #[inline(always)]
    pub fn argb1555(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Argb1555)
    }
    ///Color mode ARGB4444
    #[inline(always)]
    pub fn argb4444(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Argb4444)
    }
    ///Color mode L8
    #[inline(always)]
    pub fn l8(self) -> &'a mut crate::W<REG> {
        self.variant(CM::L8)
    }
    ///Color mode AL44
    #[inline(always)]
    pub fn al44(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Al44)
    }
    ///Color mode AL88
    #[inline(always)]
    pub fn al88(self) -> &'a mut crate::W<REG> {
        self.variant(CM::Al88)
    }
    ///Color mode L4
    #[inline(always)]
    pub fn l4(self) -> &'a mut crate::W<REG> {
        self.variant(CM::L4)
    }
    ///Color mode A8
    #[inline(always)]
    pub fn a8(self) -> &'a mut crate::W<REG> {
        self.variant(CM::A8)
    }
    ///Color mode A4
    #[inline(always)]
    pub fn a4(self) -> &'a mut crate::W<REG> {
        self.variant(CM::A4)
    }
}
/**CLUT Color mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCM {
    ///0: CLUT color format ARGB8888
    Argb8888 = 0,
    ///1: CLUT color format RGB888
    Rgb888 = 1,
}
impl From<CCM> for bool {
    #[inline(always)]
    fn from(variant: CCM) -> Self {
        variant as u8 != 0
    }
}
///Field `CCM` reader - CLUT Color mode
pub type CCM_R = crate::BitReader<CCM>;
impl CCM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CCM {
        match self.bits {
            false => CCM::Argb8888,
            true => CCM::Rgb888,
        }
    }
    ///CLUT color format ARGB8888
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == CCM::Argb8888
    }
    ///CLUT color format RGB888
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == CCM::Rgb888
    }
}
///Field `CCM` writer - CLUT Color mode
pub type CCM_W<'a, REG> = crate::BitWriter<'a, REG, CCM>;
impl<'a, REG> CCM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CLUT color format ARGB8888
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut crate::W<REG> {
        self.variant(CCM::Argb8888)
    }
    ///CLUT color format RGB888
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut crate::W<REG> {
        self.variant(CCM::Rgb888)
    }
}
/**Start

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START {
    ///1: Start the automatic loading of the CLUT
    Start = 1,
}
impl From<START> for bool {
    #[inline(always)]
    fn from(variant: START) -> Self {
        variant as u8 != 0
    }
}
///Field `START` reader - Start
pub type START_R = crate::BitReader<START>;
impl START_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<START> {
        match self.bits {
            true => Some(START::Start),
            _ => None,
        }
    }
    ///Start the automatic loading of the CLUT
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START::Start
    }
}
///Field `START` writer - Start
pub type START_W<'a, REG> = crate::BitWriter<'a, REG, START>;
impl<'a, REG> START_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Start the automatic loading of the CLUT
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(START::Start)
    }
}
///Field `CS` reader - CLUT size
pub type CS_R = crate::FieldReader;
///Field `CS` writer - CLUT size
pub type CS_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
/**Alpha mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AM {
    ///0: No modification of alpha channel
    NoModify = 0,
    ///1: Replace with value in ALPHA\[7:0\]
    Replace = 1,
    ///2: Multiply with value in ALPHA\[7:0\]
    Multiply = 2,
}
impl From<AM> for u8 {
    #[inline(always)]
    fn from(variant: AM) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AM {
    type Ux = u8;
}
impl crate::IsEnum for AM {}
///Field `AM` reader - Alpha mode
pub type AM_R = crate::FieldReader<AM>;
impl AM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<AM> {
        match self.bits {
            0 => Some(AM::NoModify),
            1 => Some(AM::Replace),
            2 => Some(AM::Multiply),
            _ => None,
        }
    }
    ///No modification of alpha channel
    #[inline(always)]
    pub fn is_no_modify(&self) -> bool {
        *self == AM::NoModify
    }
    ///Replace with value in ALPHA\[7:0\]
    #[inline(always)]
    pub fn is_replace(&self) -> bool {
        *self == AM::Replace
    }
    ///Multiply with value in ALPHA\[7:0\]
    #[inline(always)]
    pub fn is_multiply(&self) -> bool {
        *self == AM::Multiply
    }
}
///Field `AM` writer - Alpha mode
pub type AM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, AM>;
impl<'a, REG> AM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No modification of alpha channel
    #[inline(always)]
    pub fn no_modify(self) -> &'a mut crate::W<REG> {
        self.variant(AM::NoModify)
    }
    ///Replace with value in ALPHA\[7:0\]
    #[inline(always)]
    pub fn replace(self) -> &'a mut crate::W<REG> {
        self.variant(AM::Replace)
    }
    ///Multiply with value in ALPHA\[7:0\]
    #[inline(always)]
    pub fn multiply(self) -> &'a mut crate::W<REG> {
        self.variant(AM::Multiply)
    }
}
///Field `ALPHA` reader - Alpha value
pub type ALPHA_R = crate::FieldReader;
///Field `ALPHA` writer - Alpha value
pub type ALPHA_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bits 0:3 - Color mode
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - CLUT Color mode
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Start
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:15 - CLUT size
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:17 - Alpha mode
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 24:31 - Alpha value
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BGPFCCR")
            .field("alpha", &self.alpha())
            .field("am", &self.am())
            .field("cs", &self.cs())
            .field("start", &self.start())
            .field("ccm", &self.ccm())
            .field("cm", &self.cm())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Color mode
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W<BGPFCCRrs> {
        CM_W::new(self, 0)
    }
    ///Bit 4 - CLUT Color mode
    #[inline(always)]
    pub fn ccm(&mut self) -> CCM_W<BGPFCCRrs> {
        CCM_W::new(self, 4)
    }
    ///Bit 5 - Start
    #[inline(always)]
    pub fn start(&mut self) -> START_W<BGPFCCRrs> {
        START_W::new(self, 5)
    }
    ///Bits 8:15 - CLUT size
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W<BGPFCCRrs> {
        CS_W::new(self, 8)
    }
    ///Bits 16:17 - Alpha mode
    #[inline(always)]
    pub fn am(&mut self) -> AM_W<BGPFCCRrs> {
        AM_W::new(self, 16)
    }
    ///Bits 24:31 - Alpha value
    #[inline(always)]
    pub fn alpha(&mut self) -> ALPHA_W<BGPFCCRrs> {
        ALPHA_W::new(self, 24)
    }
}
/**background PFC control register

You can [`read`](crate::Reg::read) this register and get [`bgpfccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgpfccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F746.html#DMA2D:BGPFCCR)*/
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
