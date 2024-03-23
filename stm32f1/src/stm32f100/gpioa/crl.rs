#[doc = "Register `CRL` reader"]
pub type R = crate::R<CRLrs>;
#[doc = "Register `CRL` writer"]
pub type W = crate::W<CRLrs>;
#[doc = "Port n.%s mode bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE0 {
    #[doc = "0: Input mode (reset state)"]
    Input = 0,
    #[doc = "1: Output mode 10 MHz"]
    Output = 1,
    #[doc = "2: Output mode 2 MHz"]
    Output2 = 2,
    #[doc = "3: Output mode 50 MHz"]
    Output50 = 3,
}
impl From<MODE0> for u8 {
    #[inline(always)]
    fn from(variant: MODE0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE0 {
    type Ux = u8;
}
#[doc = "Field `MODE(0-7)` reader - Port n.%s mode bits"]
pub type MODE_R = crate::FieldReader<MODE0>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE0 {
        match self.bits {
            0 => MODE0::Input,
            1 => MODE0::Output,
            2 => MODE0::Output2,
            3 => MODE0::Output50,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE0::Input
    }
    #[doc = "Output mode 10 MHz"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == MODE0::Output
    }
    #[doc = "Output mode 2 MHz"]
    #[inline(always)]
    pub fn is_output2(&self) -> bool {
        *self == MODE0::Output2
    }
    #[doc = "Output mode 50 MHz"]
    #[inline(always)]
    pub fn is_output50(&self) -> bool {
        *self == MODE0::Output50
    }
}
#[doc = "Field `MODE(0-7)` writer - Port n.%s mode bits"]
pub type MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MODE0>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Input)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Output)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline(always)]
    pub fn output2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Output2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline(always)]
    pub fn output50(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Output50)
    }
}
#[doc = "Port n.%s configuration bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CNF0 {
    #[doc = "0: Analog mode / Push-Pull mode"]
    PushPull = 0,
    #[doc = "1: Floating input (reset state) / Open Drain-Mode"]
    OpenDrain = 1,
    #[doc = "2: Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    AltPushPull = 2,
    #[doc = "3: Alternate Function Open-Drain Mode"]
    AltOpenDrain = 3,
}
impl From<CNF0> for u8 {
    #[inline(always)]
    fn from(variant: CNF0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CNF0 {
    type Ux = u8;
}
#[doc = "Field `CNF(0-7)` reader - Port n.%s configuration bits"]
pub type CNF_R = crate::FieldReader<CNF0>;
impl CNF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CNF0 {
        match self.bits {
            0 => CNF0::PushPull,
            1 => CNF0::OpenDrain,
            2 => CNF0::AltPushPull,
            3 => CNF0::AltOpenDrain,
            _ => unreachable!(),
        }
    }
    #[doc = "Analog mode / Push-Pull mode"]
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        *self == CNF0::PushPull
    }
    #[doc = "Floating input (reset state) / Open Drain-Mode"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == CNF0::OpenDrain
    }
    #[doc = "Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    #[inline(always)]
    pub fn is_alt_push_pull(&self) -> bool {
        *self == CNF0::AltPushPull
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline(always)]
    pub fn is_alt_open_drain(&self) -> bool {
        *self == CNF0::AltOpenDrain
    }
}
#[doc = "Field `CNF(0-7)` writer - Port n.%s configuration bits"]
pub type CNF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CNF0>;
impl<'a, REG> CNF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Analog mode / Push-Pull mode"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut crate::W<REG> {
        self.variant(CNF0::PushPull)
    }
    #[doc = "Floating input (reset state) / Open Drain-Mode"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(CNF0::OpenDrain)
    }
    #[doc = "Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    #[inline(always)]
    pub fn alt_push_pull(self) -> &'a mut crate::W<REG> {
        self.variant(CNF0::AltPushPull)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline(always)]
    pub fn alt_open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(CNF0::AltOpenDrain)
    }
}
impl R {
    #[doc = "Port n.(0-7) mode bits"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `MODE0` field"]
    #[inline(always)]
    pub fn mode(&self, n: u8) -> MODE_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        MODE_R::new(((self.bits >> (n * 4)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Port n.(0-7) mode bits"]
    #[inline(always)]
    pub fn mode_iter(&self) -> impl Iterator<Item = MODE_R> + '_ {
        (0..8).map(move |n| MODE_R::new(((self.bits >> (n * 4)) & 3) as u8))
    }
    #[doc = "Bits 0:1 - Port n.0 mode bits"]
    #[inline(always)]
    pub fn mode0(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port n.1 mode bits"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port n.2 mode bits"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port n.3 mode bits"]
    #[inline(always)]
    pub fn mode3(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port n.4 mode bits"]
    #[inline(always)]
    pub fn mode4(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port n.5 mode bits"]
    #[inline(always)]
    pub fn mode5(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port n.6 mode bits"]
    #[inline(always)]
    pub fn mode6(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port n.7 mode bits"]
    #[inline(always)]
    pub fn mode7(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Port n.(0-7) configuration bits"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CNF0` field"]
    #[inline(always)]
    pub fn cnf(&self, n: u8) -> CNF_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CNF_R::new(((self.bits >> (n * 4 + 2)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Port n.(0-7) configuration bits"]
    #[inline(always)]
    pub fn cnf_iter(&self) -> impl Iterator<Item = CNF_R> + '_ {
        (0..8).map(move |n| CNF_R::new(((self.bits >> (n * 4 + 2)) & 3) as u8))
    }
    #[doc = "Bits 2:3 - Port n.0 configuration bits"]
    #[inline(always)]
    pub fn cnf0(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port n.1 configuration bits"]
    #[inline(always)]
    pub fn cnf1(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port n.2 configuration bits"]
    #[inline(always)]
    pub fn cnf2(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port n.3 configuration bits"]
    #[inline(always)]
    pub fn cnf3(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port n.4 configuration bits"]
    #[inline(always)]
    pub fn cnf4(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port n.5 configuration bits"]
    #[inline(always)]
    pub fn cnf5(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port n.6 configuration bits"]
    #[inline(always)]
    pub fn cnf6(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port n.7 configuration bits"]
    #[inline(always)]
    pub fn cnf7(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Port n.(0-7) mode bits"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `MODE0` field"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self, n: u8) -> MODE_W<CRLrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        MODE_W::new(self, n * 4)
    }
    #[doc = "Bits 0:1 - Port n.0 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode0(&mut self) -> MODE_W<CRLrs> {
        MODE_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Port n.1 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE_W<CRLrs> {
        MODE_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Port n.2 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> MODE_W<CRLrs> {
        MODE_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - Port n.3 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode3(&mut self) -> MODE_W<CRLrs> {
        MODE_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - Port n.4 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode4(&mut self) -> MODE_W<CRLrs> {
        MODE_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Port n.5 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode5(&mut self) -> MODE_W<CRLrs> {
        MODE_W::new(self, 20)
    }
    #[doc = "Bits 24:25 - Port n.6 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode6(&mut self) -> MODE_W<CRLrs> {
        MODE_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - Port n.7 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode7(&mut self) -> MODE_W<CRLrs> {
        MODE_W::new(self, 28)
    }
    #[doc = "Port n.(0-7) configuration bits"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CNF0` field"]
    #[inline(always)]
    #[must_use]
    pub fn cnf(&mut self, n: u8) -> CNF_W<CRLrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CNF_W::new(self, n * 4 + 2)
    }
    #[doc = "Bits 2:3 - Port n.0 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf0(&mut self) -> CNF_W<CRLrs> {
        CNF_W::new(self, 2)
    }
    #[doc = "Bits 6:7 - Port n.1 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf1(&mut self) -> CNF_W<CRLrs> {
        CNF_W::new(self, 6)
    }
    #[doc = "Bits 10:11 - Port n.2 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf2(&mut self) -> CNF_W<CRLrs> {
        CNF_W::new(self, 10)
    }
    #[doc = "Bits 14:15 - Port n.3 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf3(&mut self) -> CNF_W<CRLrs> {
        CNF_W::new(self, 14)
    }
    #[doc = "Bits 18:19 - Port n.4 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf4(&mut self) -> CNF_W<CRLrs> {
        CNF_W::new(self, 18)
    }
    #[doc = "Bits 22:23 - Port n.5 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf5(&mut self) -> CNF_W<CRLrs> {
        CNF_W::new(self, 22)
    }
    #[doc = "Bits 26:27 - Port n.6 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf6(&mut self) -> CNF_W<CRLrs> {
        CNF_W::new(self, 26)
    }
    #[doc = "Bits 30:31 - Port n.7 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf7(&mut self) -> CNF_W<CRLrs> {
        CNF_W::new(self, 30)
    }
}
#[doc = "Port configuration register low (GPIOn_CRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRLrs;
impl crate::RegisterSpec for CRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crl::R`](R) reader structure"]
impl crate::Readable for CRLrs {}
#[doc = "`write(|w| ..)` method takes [`crl::W`](W) writer structure"]
impl crate::Writable for CRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRL to value 0x4444_4444"]
impl crate::Resettable for CRLrs {
    const RESET_VALUE: u32 = 0x4444_4444;
}
