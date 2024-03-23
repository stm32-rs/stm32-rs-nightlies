#[doc = "Register `CRH` reader"]
pub type R = crate::R<CRHrs>;
#[doc = "Register `CRH` writer"]
pub type W = crate::W<CRHrs>;
#[doc = "Port n.%s mode bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE8 {
    #[doc = "0: Input mode (reset state)"]
    Input = 0,
    #[doc = "1: Output mode 10 MHz"]
    Output = 1,
    #[doc = "2: Output mode 2 MHz"]
    Output2 = 2,
    #[doc = "3: Output mode 50 MHz"]
    Output50 = 3,
}
impl From<MODE8> for u8 {
    #[inline(always)]
    fn from(variant: MODE8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE8 {
    type Ux = u8;
}
#[doc = "Field `MODE(8-15)` reader - Port n.%s mode bits"]
pub type MODE_R = crate::FieldReader<MODE8>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE8 {
        match self.bits {
            0 => MODE8::Input,
            1 => MODE8::Output,
            2 => MODE8::Output2,
            3 => MODE8::Output50,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE8::Input
    }
    #[doc = "Output mode 10 MHz"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == MODE8::Output
    }
    #[doc = "Output mode 2 MHz"]
    #[inline(always)]
    pub fn is_output2(&self) -> bool {
        *self == MODE8::Output2
    }
    #[doc = "Output mode 50 MHz"]
    #[inline(always)]
    pub fn is_output50(&self) -> bool {
        *self == MODE8::Output50
    }
}
#[doc = "Field `MODE(8-15)` writer - Port n.%s mode bits"]
pub type MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MODE8>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8::Input)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8::Output)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline(always)]
    pub fn output2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8::Output2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline(always)]
    pub fn output50(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8::Output50)
    }
}
#[doc = "Port n.%s configuration bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CNF8 {
    #[doc = "0: Analog mode / Push-Pull mode"]
    PushPull = 0,
    #[doc = "1: Floating input (reset state) / Open Drain-Mode"]
    OpenDrain = 1,
    #[doc = "2: Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    AltPushPull = 2,
    #[doc = "3: Alternate Function Open-Drain Mode"]
    AltOpenDrain = 3,
}
impl From<CNF8> for u8 {
    #[inline(always)]
    fn from(variant: CNF8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CNF8 {
    type Ux = u8;
}
#[doc = "Field `CNF(8-15)` reader - Port n.%s configuration bits"]
pub type CNF_R = crate::FieldReader<CNF8>;
impl CNF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CNF8 {
        match self.bits {
            0 => CNF8::PushPull,
            1 => CNF8::OpenDrain,
            2 => CNF8::AltPushPull,
            3 => CNF8::AltOpenDrain,
            _ => unreachable!(),
        }
    }
    #[doc = "Analog mode / Push-Pull mode"]
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        *self == CNF8::PushPull
    }
    #[doc = "Floating input (reset state) / Open Drain-Mode"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == CNF8::OpenDrain
    }
    #[doc = "Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    #[inline(always)]
    pub fn is_alt_push_pull(&self) -> bool {
        *self == CNF8::AltPushPull
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline(always)]
    pub fn is_alt_open_drain(&self) -> bool {
        *self == CNF8::AltOpenDrain
    }
}
#[doc = "Field `CNF(8-15)` writer - Port n.%s configuration bits"]
pub type CNF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CNF8>;
impl<'a, REG> CNF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Analog mode / Push-Pull mode"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut crate::W<REG> {
        self.variant(CNF8::PushPull)
    }
    #[doc = "Floating input (reset state) / Open Drain-Mode"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(CNF8::OpenDrain)
    }
    #[doc = "Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    #[inline(always)]
    pub fn alt_push_pull(self) -> &'a mut crate::W<REG> {
        self.variant(CNF8::AltPushPull)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline(always)]
    pub fn alt_open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(CNF8::AltOpenDrain)
    }
}
impl R {
    #[doc = "Port n.(8-15) mode bits"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `MODE8` field"]
    #[inline(always)]
    pub fn mode(&self, n: u8) -> MODE_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        MODE_R::new(((self.bits >> (n * 4)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Port n.(8-15) mode bits"]
    #[inline(always)]
    pub fn mode_iter(&self) -> impl Iterator<Item = MODE_R> + '_ {
        (0..8).map(move |n| MODE_R::new(((self.bits >> (n * 4)) & 3) as u8))
    }
    #[doc = "Bits 0:1 - Port n.8 mode bits"]
    #[inline(always)]
    pub fn mode8(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port n.9 mode bits"]
    #[inline(always)]
    pub fn mode9(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port n.10 mode bits"]
    #[inline(always)]
    pub fn mode10(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port n.11 mode bits"]
    #[inline(always)]
    pub fn mode11(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port n.12 mode bits"]
    #[inline(always)]
    pub fn mode12(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port n.13 mode bits"]
    #[inline(always)]
    pub fn mode13(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port n.14 mode bits"]
    #[inline(always)]
    pub fn mode14(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port n.15 mode bits"]
    #[inline(always)]
    pub fn mode15(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Port n.(8-15) configuration bits"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CNF8` field"]
    #[inline(always)]
    pub fn cnf(&self, n: u8) -> CNF_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CNF_R::new(((self.bits >> (n * 4 + 2)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Port n.(8-15) configuration bits"]
    #[inline(always)]
    pub fn cnf_iter(&self) -> impl Iterator<Item = CNF_R> + '_ {
        (0..8).map(move |n| CNF_R::new(((self.bits >> (n * 4 + 2)) & 3) as u8))
    }
    #[doc = "Bits 2:3 - Port n.8 configuration bits"]
    #[inline(always)]
    pub fn cnf8(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port n.9 configuration bits"]
    #[inline(always)]
    pub fn cnf9(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port n.10 configuration bits"]
    #[inline(always)]
    pub fn cnf10(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port n.11 configuration bits"]
    #[inline(always)]
    pub fn cnf11(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port n.12 configuration bits"]
    #[inline(always)]
    pub fn cnf12(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port n.13 configuration bits"]
    #[inline(always)]
    pub fn cnf13(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port n.14 configuration bits"]
    #[inline(always)]
    pub fn cnf14(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port n.15 configuration bits"]
    #[inline(always)]
    pub fn cnf15(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Port n.(8-15) mode bits"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `MODE8` field"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self, n: u8) -> MODE_W<CRHrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        MODE_W::new(self, n * 4)
    }
    #[doc = "Bits 0:1 - Port n.8 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode8(&mut self) -> MODE_W<CRHrs> {
        MODE_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Port n.9 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode9(&mut self) -> MODE_W<CRHrs> {
        MODE_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Port n.10 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode10(&mut self) -> MODE_W<CRHrs> {
        MODE_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - Port n.11 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode11(&mut self) -> MODE_W<CRHrs> {
        MODE_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - Port n.12 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode12(&mut self) -> MODE_W<CRHrs> {
        MODE_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Port n.13 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode13(&mut self) -> MODE_W<CRHrs> {
        MODE_W::new(self, 20)
    }
    #[doc = "Bits 24:25 - Port n.14 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode14(&mut self) -> MODE_W<CRHrs> {
        MODE_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - Port n.15 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode15(&mut self) -> MODE_W<CRHrs> {
        MODE_W::new(self, 28)
    }
    #[doc = "Port n.(8-15) configuration bits"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CNF8` field"]
    #[inline(always)]
    #[must_use]
    pub fn cnf(&mut self, n: u8) -> CNF_W<CRHrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CNF_W::new(self, n * 4 + 2)
    }
    #[doc = "Bits 2:3 - Port n.8 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf8(&mut self) -> CNF_W<CRHrs> {
        CNF_W::new(self, 2)
    }
    #[doc = "Bits 6:7 - Port n.9 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf9(&mut self) -> CNF_W<CRHrs> {
        CNF_W::new(self, 6)
    }
    #[doc = "Bits 10:11 - Port n.10 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf10(&mut self) -> CNF_W<CRHrs> {
        CNF_W::new(self, 10)
    }
    #[doc = "Bits 14:15 - Port n.11 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf11(&mut self) -> CNF_W<CRHrs> {
        CNF_W::new(self, 14)
    }
    #[doc = "Bits 18:19 - Port n.12 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf12(&mut self) -> CNF_W<CRHrs> {
        CNF_W::new(self, 18)
    }
    #[doc = "Bits 22:23 - Port n.13 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf13(&mut self) -> CNF_W<CRHrs> {
        CNF_W::new(self, 22)
    }
    #[doc = "Bits 26:27 - Port n.14 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf14(&mut self) -> CNF_W<CRHrs> {
        CNF_W::new(self, 26)
    }
    #[doc = "Bits 30:31 - Port n.15 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf15(&mut self) -> CNF_W<CRHrs> {
        CNF_W::new(self, 30)
    }
}
#[doc = "Port configuration register high (GPIOn_CRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRHrs;
impl crate::RegisterSpec for CRHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crh::R`](R) reader structure"]
impl crate::Readable for CRHrs {}
#[doc = "`write(|w| ..)` method takes [`crh::W`](W) writer structure"]
impl crate::Writable for CRHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRH to value 0x4444_4444"]
impl crate::Resettable for CRHrs {
    const RESET_VALUE: u32 = 0x4444_4444;
}
